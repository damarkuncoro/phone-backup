use anyhow::Result;
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};

use crate::storage::store::ObjectStoreKey;
use tracing::{info, instrument, warn};

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

#[derive(Debug, Default, serde::Serialize)]
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
    #[instrument(skip(self, encryption))]
    pub fn verify_repository(
        &self,
        encryption: domain::EncryptionMode,
    ) -> Result<VerificationReport> {
        let devices = self.repository.list_devices()?;
        let mut report = VerificationReport::default();

        for device in devices {
            let files = self.repository.list_files(&device.id)?;
            for file in files {
                report.total_files += 1;

                // Check if file is chunked
                let chunks = self.repository.get_file_chunks(&file.id)?;
                if !chunks.is_empty() {
                    let mut all_chunks_present = true;
                    for (chunk_id, _offset, _length, storage_key) in chunks {
                        let object_path = ObjectStoreKey::compute_object_path_v4(&storage_key);
                        if !self.storage.exists(&object_path)? {
                            all_chunks_present = false;
                            report.missing_objects.push(format!("chunk:{}", chunk_id));
                        }
                    }
                    if all_chunks_present {
                        report.verified_files += 1;
                    }
                    continue;
                }

                // Standard single object check
                let hash = match file.hash_sha256 {
                    Some(h) => h,
                    None => {
                        report.corrupted_files.push(file.path);
                        continue;
                    }
                };

                let object_id = ObjectStoreKey::compute_object_id(
                    &hash,
                    Some(&file.mime_type),
                    encryption.is_encrypted(),
                );
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

    #[instrument(skip(self))]
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

    #[instrument(skip(self))]
    pub fn garbage_collect(&self) -> Result<u64> {
        info!("Starting Garbage Collection (V4.0)...");

        // 1. Database level pruning
        let db_deleted = self.repository.prune_orphans()?;
        info!("Pruned {} orphaned records from database.", db_deleted);

        // 2. Storage level cleanup
        let referenced_items = self.repository.get_all_referenced_hashes()?;
        let storage_paths = self.storage.list()?;

        let mut deleted_count = 0;
        for path in storage_paths {
            // Path can be:
            // - objects/v4/aa/bb/UUID
            // - manifests/UUID.json
            // - objects/aa/bb/HASH (Legacy)

            let mut is_referenced = false;

            // Check manifest matches
            if path.starts_with("manifests/") {
                if referenced_items.contains(&path) {
                    is_referenced = true;
                }
            } else {
                // Extract filename/UUID from path
                if let Some(filename) = path.split('/').next_back() {
                    // Remove extension if any
                    let base_name = filename.split('.').next().unwrap_or("");
                    if referenced_items.contains(base_name) {
                        is_referenced = true;
                    }
                }
            }

            if !is_referenced {
                warn!("Deleting orphan object from storage: {}", path);
                self.storage.delete(&path)?;
                deleted_count += 1;
            }
        }

        info!(
            "Garbage Collection finished. Deleted {} physical objects.",
            deleted_count
        );
        Ok(deleted_count)
    }
}
