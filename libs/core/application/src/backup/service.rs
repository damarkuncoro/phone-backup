use anyhow::Result;
use domain::{BackupPolicy, DeviceId, EncryptionMode, FileEntry, Snapshot, SnapshotStatus};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort,
};

use tracing::{info, instrument};

use super::guard::SnapshotGuard;
use super::planner::BackupPlanner;
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
        self.progress.log("Scanning device filesystem...");
        let all_files = self
            .scanner_adapter
            .scan(id, policy.include_paths.clone())?;
        let manifest_files: Vec<FileEntry> = all_files
            .into_iter()
            .filter(|f| policy.should_include(&f.path))
            .collect();
        self.progress.log(&format!(
            "Manifest built with {} files",
            manifest_files.len()
        ));

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

        // Determine what actually needs uploading via BackupPlanner
        let plan = BackupPlanner::build_plan(&manifest_files, &previous_files, &already_backed_up);
        let plan_msg = format!(
            "Plan: {} to upload ({:.2} MB), {} reused",
            plan.upload_count(),
            plan.upload_bytes as f64 / 1024.0 / 1024.0,
            plan.reuse_count()
        );
        info!("📊 {}", plan_msg);
        self.progress.log(&plan_msg);

        self.check_available_disk_space(plan.upload_bytes)?;

        // 4. UPLOAD CHANGED FILES (STATE TRANSITION GUARDED)
        snapshot.start()?;
        self.repository
            .create_snapshot(&snapshot)
            .or_else(|_| self.repository.update_snapshot(&snapshot))?;

        let guard = SnapshotGuard::new(&self.repository, &mut snapshot);

        self.upload_files(
            id,
            &plan.upload,
            &previous_files,
            &already_backed_up,
            guard.snapshot,
            &encryption,
        )?;

        // 5. BACKUP STRUCTURED DATA (Apps, SMS, etc.)
        self.backup_metadata_and_structured_data(id, guard.snapshot, &encryption)?;

        // 6. FINALIZE SNAPSHOT & COMMIT MANIFEST
        self.progress
            .log("Finalizing snapshot and storing immutable manifest...");
        guard.snapshot.complete()?;

        // Snapshot Commit Protocol: Write manifest before updating status in DB to ensure atomicity
        let manifest_manager =
            super::manifest::ManifestManager::new(&self.repository, &self.storage);
        manifest_manager.create_and_store_manifest(guard.snapshot)?;

        self.repository.update_snapshot(guard.snapshot)?;
        guard.mark_completed();

        // --- SMART RETENTION ---
        let _ = self.apply_retention_strategy(id, &domain::KeepCountStrategy { keep_limit: 10 });

        info!("✨ Backup Job Completed: {}", snapshot.id.0);
        self.progress.finish("Backup completed successfully!");
        Ok(snapshot)
    }

    fn check_battery_and_thermal(&self, id: &DeviceId) -> Result<()> {
        if let Ok((level, temp)) = self.device_adapter.battery_status(id) {
            if level < 10 {
                anyhow::bail!("Battery too low ({}%). Please charge your device.", level);
            }
            if temp > 45.0 {
                anyhow::bail!(
                    "Device temperature too high ({:.1}°C). Let it cool down.",
                    temp
                );
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
        data_type: domain::StructuredDataType,
        data: &V,
        encryption: &EncryptionMode,
    ) -> Result<()> {
        let json = serde_json::to_vec(data)?;
        let object_manager = crate::storage::manager::ObjectManager::new(
            &self.storage,
            &self.repository,
            encryption,
        );

        let (chunk_id, _, _) = object_manager.put_object(&json, None)?;

        self.repository
            .save_structured_data_ref(snapshot_id, data_type, &chunk_id)?;
        Ok(())
    }
}
