use super::BackupService;
use crate::storage::manager::ObjectManager;
use anyhow::Result;
use domain::{EncryptionMode, SnapshotId};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};
use std::fs;
use std::path::Path;
use tracing::instrument;

impl<D, S, R, T, A, DP, P> BackupService<D, S, R, T, A, DP, P>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
{
    pub fn perform_restore(
        &self,
        snapshot_id: &SnapshotId,
        target_dir: &str,
        encryption: EncryptionMode,
        filters: Option<Vec<String>>,
    ) -> Result<()> {
        let options = domain::RestoreOptions {
            target_dir: target_dir.to_string(),
            encryption,
            filters,
            overwrite_existing: true,
        };
        self.perform_restore_with_options(snapshot_id, &options)
    }

    #[instrument(skip(self, options))]
    pub fn perform_restore_with_options(
        &self,
        snapshot_id: &SnapshotId,
        options: &domain::RestoreOptions,
    ) -> Result<()> {
        use rayon::prelude::*;

        let files = self.repository.get_snapshot_files(snapshot_id)?;
        let target_base = Path::new(&options.target_dir);
        let object_manager =
            ObjectManager::new(&self.storage, &self.repository, &options.encryption);

        self.progress
            .start(files.len() as u64, "Starting restoration...");

        files.into_par_iter().try_for_each(|file| {
            if let Some(ref f_list) = options.filters {
                let matches = f_list
                    .iter()
                    .any(|f| file.path.starts_with(f) || file.path.contains(f));
                if !matches {
                    self.progress.inc(1, "Skipping...");
                    return Ok(());
                }
            }

            let relative_path = file.path.strip_prefix('/').unwrap_or(&file.path);
            let restore_path = target_base.join(relative_path);

            if !options.overwrite_existing && restore_path.exists() {
                self.progress
                    .inc(1, &format!("Skipped existing {}", file.name));
                return Ok(());
            }

            self.progress.inc(0, &format!("Restoring {}", file.name));

            let chunks = self.repository.get_file_chunks(&file.id)?;
            let data = if !chunks.is_empty() {
                let mut full_data = Vec::with_capacity(file.size_bytes as usize);
                for (chunk_id, _offset, _length, _storage_key) in chunks {
                    let chunk_data = object_manager.get_chunk(&chunk_id)?;
                    full_data.extend(chunk_data);
                }
                full_data
            } else {
                // Fallback for old style backups or structured data
                let hash = file
                    .hash_sha256
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("File {} has no hash", file.path))?;

                object_manager.get_chunk(hash)?
            };

            if let Some(parent) = restore_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(restore_path, data)?;

            self.progress.inc(1, &format!("Restored {}", file.name));
            Ok::<(), anyhow::Error>(())
        })?;

        self.progress.finish("Restoration complete.");
        Ok(())
    }
}
