use anyhow::Result;
use chrono::Utc;
use domain::{BackupPolicy, DeviceId, FileEntry, Snapshot, SnapshotStatus, EncryptionMode};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};

use crate::hashing::calculate_hash;
use crate::object_store::ObjectStoreKey;
use crate::security::EncryptionEngine;
use tracing::{info, instrument};

use super::BackupService;

impl<
        D: DevicePort,
        S: ScannerPort,
        R: RepositoryPort,
        T: StoragePort,
        A: AppProviderPort,
        DP: DataProviderPort,
        P: ports::ProgressPort,
    > BackupService<D, S, R, T, A, DP, P>
{
    /// Perform a full or incremental backup of a device (Phase 07-21 + Storage Check + Resume + Asymmetric Crypto)
    #[instrument(skip(self, policy))]
    pub fn perform_backup(
        &self,
        id: &DeviceId,
        encryption: EncryptionMode,
        policy: Option<BackupPolicy>,
    ) -> Result<Snapshot> {
        info!("🚀 Starting Backup Job for device: {}", id.0);
        let policy = policy.unwrap_or_default();

        // 1. SAFETY CHECK
        let device = self.device_adapter.info(id)?;
        self.repository.save_device(&device)?;
        self.check_battery_and_thermal(id)?;

        // 2. SCAN DEVICE
        let all_files = self.scanner_adapter.scan(id, policy.include_paths.clone())?;
        let manifest_files: Vec<FileEntry> = all_files
            .into_iter()
            .filter(|f| policy.should_include(&f.path))
            .collect();
        info!("📋 Manifest built with {} files", manifest_files.len());

        // 3. COMPARE PREVIOUS COMPLETED BACKUP (DIFFING)
        let latest_completed_snapshot = self.repository.get_latest_completed_snapshot(id)?;
        let mut previous_files = std::collections::HashMap::new();
        if let Some(ref snapshot) = latest_completed_snapshot {
            for f in self.repository.get_snapshot_files(&snapshot.id)? {
                previous_files.insert(f.path.clone(), f);
            }
        }

        let mut snapshot = if let Some(incomplete) = self.repository.get_resumable_snapshot(id)? {
            info!("🔄 Resuming interrupted snapshot: {}", incomplete.id.0);
            incomplete
        } else {
            Snapshot::new(id.clone())
        };

        let already_backed_up: std::collections::HashSet<String> = self
            .repository
            .get_snapshot_files(&snapshot.id)?
            .into_iter()
            .map(|f| f.path)
            .collect();

        // Determine what actually needs uploading
        let files_to_upload: Vec<FileEntry> = manifest_files
            .iter()
            .cloned()
            .filter(|f| !already_backed_up.contains(&f.path))
            .filter(|f| {
                if let Some(prev) = previous_files.get(&f.path) {
                    !(prev.size_bytes == f.size_bytes && prev.modified_at == f.modified_at)
                } else {
                    true
                }
            })
            .collect();

        let total_required: u64 = files_to_upload.iter().map(|f| f.size_bytes).sum();
        self.check_available_disk_space(total_required)?;

        // 4. UPLOAD CHANGED FILES
        snapshot.status = SnapshotStatus::Running;
        self.repository
            .create_snapshot(&snapshot)
            .or_else(|_| self.repository.update_snapshot(&snapshot))?;

        self.upload_files(id, &manifest_files, &previous_files, &already_backed_up, &mut snapshot, &encryption)?;

        // 5. BACKUP STRUCTURED DATA (Apps, SMS, etc.)
        self.backup_metadata_and_structured_data(id, &mut snapshot, &encryption)?;

        // 6. FINALIZE SNAPSHOT
        snapshot.status = SnapshotStatus::Completed;
        snapshot.finished_at = Some(Utc::now());
        self.repository.update_snapshot(&snapshot)?;

        // --- SMART RETENTION ---
        let _ = self.apply_retention_strategy(id, &domain::KeepCountStrategy { keep_limit: 10 });

        info!("✨ Backup Job Completed: {}", snapshot.id.0);
        Ok(snapshot)
    }

    fn check_battery_and_thermal(&self, id: &DeviceId) -> Result<()> {
        if let Ok((level, temp)) = self.device_adapter.battery_status(id) {
            if level < 10 {
                anyhow::bail!("Battery too low ({}%). Please charge your device.", level);
            }
            if temp > 45.0 {
                anyhow::bail!("Device temperature too high ({:.1}°C). Let it cool down.", temp);
            }
            info!("Safety Check: Battery {}%, Temp {}°C - OK", level, temp);
        }
        Ok(())
    }

    pub(crate) fn mark_interrupted(
        &self,
        snapshot: &mut Snapshot,
        files: u64,
        bytes: u64,
        dedup: u64,
    ) -> Result<()> {
        snapshot.status = SnapshotStatus::Interrupted;
        snapshot.total_files = files;
        snapshot.total_bytes = bytes;
        snapshot.deduped_bytes = dedup;
        self.repository.update_snapshot(snapshot)?;
        Ok(())
    }

    pub(crate) fn check_available_disk_space(&self, required_bytes: u64) -> Result<()> {
        let available = self.storage.available_space()?;
        if available < required_bytes {
            anyhow::bail!(
                "Insufficient disk space on target storage. Required: {:.2} MB, Available: {:.2} MB",
                required_bytes as f64 / 1024.0 / 1024.0,
                available as f64 / 1024.0 / 1024.0
            );
        }
        info!(
            "Target Storage Capacity Check: OK (Available: {:.2} MB, Required: {:.2} MB)",
            available as f64 / 1024.0 / 1024.0,
            required_bytes as f64 / 1024.0 / 1024.0
        );
        Ok(())
    }

    pub(crate) fn store_structured_data<V: serde::Serialize>(
        &self,
        snapshot_id: &domain::SnapshotId,
        data_type: &str,
        data: &V,
        encryption: &EncryptionMode,
    ) -> Result<()> {
        let json = serde_json::to_vec(data)?;
        let hash = calculate_hash(&json);

        let mut object_id = format!("{}.json", hash);
        if encryption.is_encrypted() {
            object_id = format!("{}.enc", object_id);
        }

        let object_path = ObjectStoreKey::compute_object_path(&hash, &object_id);

        if !self.storage.exists(&object_path)? {
            let mut data_to_write = json;

            data_to_write = match encryption {
                EncryptionMode::Password(pwd) => EncryptionEngine::encrypt(&data_to_write, pwd)?,
                EncryptionMode::PublicKey(pk) => EncryptionEngine::encrypt_with_key(&data_to_write, pk)?,
                EncryptionMode::None => data_to_write,
            };

            self.storage.write(&object_path, &mut std::io::Cursor::new(data_to_write))?;
        }

        self.repository
            .save_structured_data_ref(snapshot_id, data_type, &object_path)?;
        Ok(())
    }
}
