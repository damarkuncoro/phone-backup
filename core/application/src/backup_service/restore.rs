use anyhow::Result;
use domain::{SnapshotId, EncryptionMode};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};
use std::fs;
use std::io::Read;
use std::path::Path;

use crate::compression::CompressionEngine;
use crate::object_store::ObjectStoreKey;
use crate::security::EncryptionEngine;

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
    pub fn perform_restore(
        &self,
        snapshot_id: &SnapshotId,
        target_dir: &str,
        encryption: EncryptionMode,
        filter: Option<&str>,
    ) -> Result<()> {
        let files = self.repository.get_snapshot_files(snapshot_id)?;
        let target_base = Path::new(target_dir);

        for file in files {
            if let Some(f) = filter {
                if !file.path.contains(f) && !file.name.contains(f) {
                    continue;
                }
            }

            let hash = file
                .hash_sha256
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("File {} has no hash", file.path))?;

            let object_id = ObjectStoreKey::compute_object_id(hash, Some(&file.mime_type), encryption.is_encrypted());
            let object_path = ObjectStoreKey::compute_object_path(hash, &object_id);

            let mut reader = self.storage.read(&object_path)?;
            let mut data = Vec::new();
            reader.read_to_end(&mut data)?;

            if object_id.ends_with(".enc") {
                data = match &encryption {
                    EncryptionMode::Password(pwd) => EncryptionEngine::decrypt(&data, pwd)?,
                    EncryptionMode::PublicKey(sk) => EncryptionEngine::decrypt_with_key(&data, sk)?,
                    EncryptionMode::None => anyhow::bail!("Data is encrypted but no decryption key/password provided"),
                };
            }

            if object_id.contains(".zst") {
                data = CompressionEngine::decompress(&data)?;
            }

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
