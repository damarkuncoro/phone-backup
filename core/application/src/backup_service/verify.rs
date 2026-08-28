use anyhow::Result;
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};

use crate::object_store::ObjectStoreKey;
use super::BackupService;

#[derive(Debug, Default)]
pub struct VerificationReport {
    pub total_files: u64,
    pub verified_files: u64,
    pub missing_objects: Vec<String>,
    pub corrupted_files: Vec<String>,
}

impl VerificationReport {
    pub fn is_healthy(&self) -> bool {
        self.missing_objects.is_empty() && self.corrupted_files.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct StorageStats {
    pub total_devices: u64,
    pub total_snapshots: u64,
    pub total_logical_bytes: u64,
    pub total_deduped_bytes: u64,
}

impl StorageStats {
    pub fn efficiency_ratio(&self) -> f64 {
        if self.total_logical_bytes == 0 {
            return 1.0;
        }
        (self.total_deduped_bytes as f64 / self.total_logical_bytes as f64) * 100.0
    }
}

impl<
        D: DevicePort,
        S: ScannerPort,
        R: RepositoryPort,
        T: StoragePort,
        A: AppProviderPort,
        DP: DataProviderPort,
    > BackupService<D, S, R, T, A, DP>
{
    pub fn verify_repository(&self, encryption: domain::EncryptionMode) -> Result<VerificationReport> {
        let devices = self.repository.list_devices()?;
        let mut report = VerificationReport::default();

        for device in devices {
            let files = self.repository.list_files(&device.id)?;
            for file in files {
                report.total_files += 1;
                let hash = match file.hash_sha256 {
                    Some(h) => h,
                    None => {
                        report.corrupted_files.push(file.path);
                        continue;
                    }
                };

                let object_id = ObjectStoreKey::compute_object_id(&hash, Some(&file.mime_type), encryption.is_encrypted());
                let object_path = ObjectStoreKey::compute_object_path(&hash, &object_id);

                if !self.storage.exists(&object_path)? {
                    report.missing_objects.push(file.path);
                    continue;
                }

                report.verified_files += 1;
            }
        }

        Ok(report)
    }

    pub fn get_storage_stats(&self) -> Result<StorageStats> {
        let mut stats = StorageStats::default();
        let devices = self.repository.list_devices()?;
        stats.total_devices = devices.len() as u64;

        for device in devices {
            let snapshots = self.repository.list_snapshots(&device.id)?;
            stats.total_snapshots += snapshots.len() as u64;
            for s in snapshots {
                stats.total_logical_bytes += s.total_bytes;
                stats.total_deduped_bytes += s.deduped_bytes;
            }
        }
        Ok(stats)
    }
}
