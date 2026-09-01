use anyhow::Result;
use domain::{DeviceId, EncryptionMode, FileEntry, Snapshot};
use ports::{AppProviderPort, DataProviderPort, RepositoryPort, StoragePort, ProgressPort, DevicePort, ScannerPort};
use super::BackupService;
use tracing::info;
use crate::storage::manager::ObjectManager;
use crate::storage::store::ObjectStoreKey;

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

        self.progress.start(files.len() as u64, "Starting file backup...");

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

        let processed_files = Mutex::new(Vec::with_capacity(files.len()));
        let all_chunks = Mutex::new(Vec::new());
        let snapshot_id = snapshot.id.clone();

        files.into_par_iter().try_for_each(|file| {
            if already_backed_up.contains(&file.path) {
                self.progress.inc(1, "Skipping already backed up");
                return Ok(());
            }

            let mut skip_content = false;
            let mut file_to_process = file.clone();

            if let Some(prev) = previous_files.get(&file_to_process.path) {
                if prev.size_bytes == file_to_process.size_bytes
                    && prev.modified_at == file_to_process.modified_at
                    && prev.hash_sha256.is_some()
                {
                    let hash = prev.hash_sha256.as_ref().unwrap();
                    let is_encrypted = encryption.is_encrypted();
                    let object_id = ObjectStoreKey::compute_object_id(hash, Some(&file_to_process.mime_type), is_encrypted);
                    let object_path = ObjectStoreKey::compute_object_path(hash, &object_id);

                    if self.storage.exists(&object_path).unwrap_or(false) {
                        file_to_process.hash_sha256 = prev.hash_sha256.clone();
                        skip_content = true;
                        deduped_bytes_atomic.fetch_add(file_to_process.size_bytes, Ordering::Relaxed);
                    }
                }
            }

            match processor.process_file(id, file_to_process, skip_content) {
                Ok((processed_file, chunks)) => {
                    self.progress.inc(1, &format!("Completed {}", processed_file.name));
                    let file_id = processed_file.id.clone();
                    processed_files.lock().unwrap().push(processed_file);
                    if !chunks.is_empty() {
                        all_chunks.lock().unwrap().push((file_id, chunks));
                    }
                    Ok(())
                }
                Err(e) => {
                    let _ = self.mark_interrupted(&mut snapshot.clone(),
                        total_files_atomic.load(Ordering::Relaxed),
                        total_bytes_atomic.load(Ordering::Relaxed),
                        deduped_bytes_atomic.load(Ordering::Relaxed));
                    Err(anyhow::anyhow!("File processing error: {}", e))
                }
            }
        })?;

        // Batch save metadata
        let entries = processed_files.into_inner().unwrap();
        let chunks_to_save = all_chunks.into_inner().unwrap();

        if !entries.is_empty() {
            info!("Saving {} file entries and their chunks in batch...", entries.len());
            self.repository.save_files_batch(&entries)?;
            let ids: Vec<domain::FileId> = entries.iter().map(|f| f.id.clone()).collect();
            self.repository.link_files_to_snapshot_batch(&snapshot_id, &ids)?;

            // Save chunks for each file
            for (file_id, chunks) in chunks_to_save {
                for (i, chunk) in chunks.into_iter().enumerate() {
                    self.repository.save_file_chunk(&file_id, &chunk.hash, chunk.offset, chunk.length, i as u32)?;
                }
            }
        }

        snapshot.total_files = total_files_atomic.load(Ordering::Relaxed);
        snapshot.total_bytes = total_bytes_atomic.load(Ordering::Relaxed);
        snapshot.deduped_bytes = deduped_bytes_atomic.load(Ordering::Relaxed);

        self.progress.log("File indexing complete. Finalizing metadata...");
        Ok(())
    }
}
