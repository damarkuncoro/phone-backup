use anyhow::Result;
use domain::{SnapshotId, EncryptionMode};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};
use std::fs;
use std::path::Path;
use crate::object_manager::ObjectManager;
use tracing::instrument;
use super::BackupService;

impl<
        D: DevicePort,
        S: ScannerPort,
        R: RepositoryPort,
        T: StoragePort,
        A: AppProviderPort,
        DP: DataProviderPort,
    > BackupService<D, S, R, T, A, DP>
{
    #[instrument(skip(self, encryption, filter))]
    pub fn perform_restore(
        &self,
        snapshot_id: &SnapshotId,
        target_dir: &str,
        encryption: EncryptionMode,
        filter: Option<&str>,
    ) -> Result<()> {
        let files = self.repository.get_snapshot_files(snapshot_id)?;
        let target_base = Path::new(target_dir);
        let object_manager = ObjectManager::new(&self.storage, &encryption);

        for file in files {
            if let Some(f) = filter {
                if !file.path.contains(f) && !file.name.contains(f) {
                    continue;
                }
            }

            let chunks = self.repository.get_file_chunks(&file.id)?;
            let data = if !chunks.is_empty() {
                let mut full_data = Vec::with_capacity(file.size_bytes as usize);
                for (chunk_hash, _offset, _length) in chunks {
                    let chunk_data = object_manager.get_object(&chunk_hash, None, false)?;
                    full_data.extend(chunk_data);
                }
                full_data
            } else {
                let hash = file.hash_sha256.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("File {} has no hash", file.path))?;

                let is_compressed = crate::compression::CompressionEngine::should_compress(&file.mime_type);
                object_manager.get_object(hash, Some(&file.mime_type), is_compressed)?
            };

            let relative_path = file.path.strip_prefix('/').unwrap_or(&file.path);
            let restore_path = target_base.join(relative_path);
            if let Some(parent) = restore_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(restore_path, data)?;
        }

        Ok(())
    }
}
