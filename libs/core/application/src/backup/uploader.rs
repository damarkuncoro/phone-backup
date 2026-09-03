use super::BackupService;
use crate::storage::manager::ObjectManager;
use crate::storage::store::ObjectStoreKey;
use anyhow::Result;
use domain::{DeviceId, EncryptionMode, FileEntry, Snapshot};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};

impl<
        D: DevicePort,
        S: ScannerPort,
        R: RepositoryPort,
        T: StoragePort,
        A: AppProviderPort,
        DP: DataProviderPort,
        P: ProgressPort,
    > BackupService<D, S, R, T, A, DP, P>
{
    pub(crate) fn upload_files(
        &self,
        id: &DeviceId,
        files: &[FileEntry],
        previous_files: &std::collections::HashMap<String, FileEntry>,
        already_backed_up: &std::collections::HashSet<String>,
        snapshot: &mut Snapshot,
        encryption: &EncryptionMode,
    ) -> Result<()> {
        use rayon::prelude::*;
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::sync::Mutex;

        self.progress
            .start(files.len() as u64, "Starting file backup...");

        let total_bytes_atomic = AtomicU64::new(snapshot.total_bytes);
        let total_files_atomic = AtomicU64::new(snapshot.total_files);
        let deduped_bytes_atomic = AtomicU64::new(snapshot.deduped_bytes);

        let object_manager = ObjectManager::new(&self.storage, &self.repository, encryption);
        let processor = super::processor::FileProcessor {
            service: self,
            object_manager,
            chunking_policy: Box::new(crate::storage::DefaultChunkingPolicy),
            total_bytes: &total_bytes_atomic,
            total_files: &total_files_atomic,
            deduped_bytes: &deduped_bytes_atomic,
        };

        const CHECKPOINT_BATCH_SIZE: usize = 50;
        let snapshot_id = snapshot.id.clone();

        for batch in files.chunks(CHECKPOINT_BATCH_SIZE) {
            self.check_battery_and_thermal(id)?;

            if let Some(ref token) = self.cancellation_token {
                if token.is_cancelled() {
                    let _ = self.mark_interrupted(
                        snapshot,
                        total_files_atomic.load(Ordering::Relaxed),
                        total_bytes_atomic.load(Ordering::Relaxed),
                        deduped_bytes_atomic.load(Ordering::Relaxed),
                    );
                    anyhow::bail!("Backup was cancelled by user");
                }
            }

            let processed_batch = Mutex::new(Vec::with_capacity(batch.len()));
            let batch_chunks = Mutex::new(Vec::new());

            batch.into_par_iter().try_for_each(|file| {
                if already_backed_up.contains(&file.path) {
                    self.progress.inc(1, "Skipping already backed up");
                    return Ok(());
                }

                let mut skip_content = false;
                let mut file_to_process = file.clone();

                if let Some(prev) = previous_files.get(&file_to_process.path) {
                    if prev.size_bytes == file_to_process.size_bytes
                        && prev.modified_at == file_to_process.modified_at
                    {
                        if let Some(hash) = prev.hash_sha256.as_ref() {
                            let is_encrypted = encryption.is_encrypted();
                            let object_id = ObjectStoreKey::compute_object_id(
                                hash,
                                Some(&file_to_process.mime_type),
                                is_encrypted,
                            );
                            let object_path = ObjectStoreKey::compute_object_path(hash, &object_id);

                            if self.storage.exists(&object_path).unwrap_or(false) {
                                file_to_process.hash_sha256 = prev.hash_sha256.clone();
                                skip_content = true;
                                deduped_bytes_atomic
                                    .fetch_add(file_to_process.size_bytes, Ordering::Relaxed);
                            }
                        }
                    }
                }

                match processor.process_file(id, file_to_process, skip_content) {
                    Ok((processed_file, chunks)) => {
                        self.progress
                            .inc(1, &format!("Completed {}", processed_file.name));
                        let file_id = processed_file.id.clone();
                        processed_batch.lock().unwrap().push(processed_file);
                        if !chunks.is_empty() {
                            batch_chunks.lock().unwrap().push((file_id, chunks));
                        }
                        Ok(())
                    }
                    Err(e) => {
                        let err_str = e.to_string();
                        // Non-fatal error tolerance for transient/inaccessible individual files
                        if err_str.contains("Permission denied")
                            || err_str.contains("No such file")
                            || err_str.contains("NotFound")
                            || err_str.contains("not found")
                        {
                            tracing::warn!("Skipping inaccessible file {}: {}", file.path, e);
                            self.progress
                                .log(&format!("Skipping inaccessible file: {}", file.name));
                            self.progress.inc(1, "Skipped inaccessible file");
                            Ok(())
                        } else {
                            let _ = self.mark_interrupted(
                                &mut snapshot.clone(),
                                total_files_atomic.load(Ordering::Relaxed),
                                total_bytes_atomic.load(Ordering::Relaxed),
                                deduped_bytes_atomic.load(Ordering::Relaxed),
                            );
                            Err(anyhow::anyhow!(
                                "File processing error on {}: {}",
                                file.path,
                                e
                            ))
                        }
                    }
                }
            })?;

            // Commit batch checkpoint to repository immediately
            let entries = processed_batch.into_inner().unwrap();
            let chunks_to_save = batch_chunks.into_inner().unwrap();

            if !entries.is_empty() {
                self.repository.save_files_batch(&entries)?;
                let ids: Vec<domain::FileId> = entries.iter().map(|f| f.id.clone()).collect();
                self.repository
                    .link_files_to_snapshot_batch(&snapshot_id, &ids)?;

                // Save chunks for each file
                for (file_id, chunks) in chunks_to_save {
                    for (i, chunk) in chunks.into_iter().enumerate() {
                        self.repository.save_file_chunk(
                            &file_id,
                            &chunk.hash,
                            chunk.offset,
                            chunk.length,
                            i as u32,
                        )?;
                    }
                }

                // Update snapshot intermediate state in database
                snapshot.total_files = total_files_atomic.load(Ordering::Relaxed);
                snapshot.total_bytes = total_bytes_atomic.load(Ordering::Relaxed);
                snapshot.deduped_bytes = deduped_bytes_atomic.load(Ordering::Relaxed);
                let _ = self.repository.update_snapshot(snapshot);
            }
        }

        snapshot.total_files = total_files_atomic.load(Ordering::Relaxed);
        snapshot.total_bytes = total_bytes_atomic.load(Ordering::Relaxed);
        snapshot.deduped_bytes = deduped_bytes_atomic.load(Ordering::Relaxed);

        self.progress
            .log("File indexing complete. Finalizing metadata...");
        Ok(())
    }
}
