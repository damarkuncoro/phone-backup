use anyhow::Result;
use chrono::Utc;
use domain::{BackupPolicy, DeviceId, FileEntry, Snapshot, SnapshotStatus, EncryptionMode};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};

use crate::hashing::calculate_hash;
use crate::object_store::ObjectStoreKey;
use crate::security::EncryptionEngine;
use tracing::{info, warn, instrument};

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
        info!("Starting perform_backup for device: {}", id.0);
        let policy = policy.unwrap_or_default();

        let device = self.device_adapter.info(id)?;
        self.repository.save_device(&device)?;

        let latest_snapshot = self.repository.get_latest_snapshot(id)?;
        let mut previous_files = std::collections::HashMap::new();
        if let Some(ref snapshot) = latest_snapshot {
            for f in self.repository.get_snapshot_files(&snapshot.id)? {
                previous_files.insert(f.path.clone(), f);
            }
        }

        let mut snapshot = if let Some(incomplete) = self.repository.get_incomplete_snapshot(id)? {
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

        snapshot.status = SnapshotStatus::Running;
        self.repository
            .create_snapshot(&snapshot)
            .or_else(|_| self.repository.update_snapshot(&snapshot))?;

        let all_files = self.scanner_adapter.scan(id, policy.include_paths.clone())?;
        let files: Vec<FileEntry> = all_files
            .into_iter()
            .filter(|f| policy.should_include(&f.path))
            .collect();

        let total_required: u64 = files
            .iter()
            .filter(|f| !already_backed_up.contains(&f.path))
            .filter(|f| {
                if let Some(prev) = previous_files.get(&f.path) {
                    !(prev.size_bytes == f.size_bytes && prev.modified_at == f.modified_at)
                } else {
                    true
                }
            })
            .map(|f| f.size_bytes)
            .sum();

        self.check_available_disk_space(total_required)?;

        use rayon::prelude::*;
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::sync::Mutex;

        self.progress.start(files.len() as u64, "Starting file backup...");

        let total_bytes_atomic = AtomicU64::new(snapshot.total_bytes);
        let total_files_atomic = AtomicU64::new(snapshot.total_files);
        let deduped_bytes_atomic = AtomicU64::new(snapshot.deduped_bytes);

        let object_manager = crate::object_manager::ObjectManager::new(&self.storage, &encryption);

        let processor = crate::backup_service::processor::FileProcessor {
            service: self,
            object_manager,
            total_bytes: &total_bytes_atomic,
            total_files: &total_files_atomic,
            deduped_bytes: &deduped_bytes_atomic,
        };

        // We need a thread-safe way to update the snapshot if interrupted
        let snapshot_mutex = Mutex::new(snapshot);
        // We need a thread-safe way to collect processed files for batch linking
        let processed_ids = Mutex::new(Vec::with_capacity(files.len()));

        let result: Result<()> = files.into_par_iter().try_for_each(|file| {
            if already_backed_up.contains(&file.path) {
                self.progress.inc(1, "Skipping already backed up file");
                return Ok(());
            }

            self.progress.log(&format!("Processing: {}", file.name));
            let mut skip_content = false;
            let mut file_to_process = file;

            if let Some(prev) = previous_files.get(&file_to_process.path) {
                if prev.size_bytes == file_to_process.size_bytes
                    && prev.modified_at == file_to_process.modified_at
                    && prev.hash_sha256.is_some()
                {
                    file_to_process.hash_sha256 = prev.hash_sha256.clone();
                    skip_content = true;
                    deduped_bytes_atomic.fetch_add(file_to_process.size_bytes, Ordering::Relaxed);
                }
            }

            match processor.process_file(id, file_to_process, skip_content) {
                Ok(processed_file) => {
                    {
                        let mut ids = processed_ids.lock().unwrap();
                        ids.push(processed_file.id.clone());
                    }
                    self.progress.inc(1, &format!("Completed {}", processed_file.name));
                    Ok(())
                }
                Err(e) => {
                    let snap = snapshot_mutex.lock().unwrap();
                    let _ = self.mark_interrupted(&mut snap.clone(),
                        total_files_atomic.load(Ordering::Relaxed),
                        total_bytes_atomic.load(Ordering::Relaxed),
                        deduped_bytes_atomic.load(Ordering::Relaxed));
                    self.progress.error(&format!("File processing error: {}", e));
                    Err(anyhow::anyhow!("File processing error: {}", e))
                }
            }
        });

        result?;

        // Batch link all processed files to the snapshot
        {
            let ids = processed_ids.into_inner().unwrap();
            let snap = snapshot_mutex.lock().unwrap();
            if !ids.is_empty() {
                self.repository.link_files_to_snapshot_batch(&snap.id, &ids)?;
            }
        }

        self.progress.finish("File backup finished.");

        let mut snapshot = snapshot_mutex.into_inner().unwrap();

        tracing::info!("Starting app list backup...");
        if let Ok(apps) = self.app_provider.list_apps(id) {
            for app in &apps {
                let _ = self.repository.save_app(app);
                let _ = self.repository.link_app_to_snapshot(&snapshot.id, &app.id);
            }
            tracing::info!("Backed up {} apps", apps.len());
        }

        tracing::info!("Starting structured data backup (Contacts, SMS, Logs)...");
        if let Err(e) = self.backup_structured_data(id, &snapshot.id, &encryption) {
            tracing::error!("Structured data backup failed: {}", e);
        } else {
            tracing::info!("Structured data backup completed successfully");
        }

        snapshot.status = SnapshotStatus::Completed;
        snapshot.finished_at = Some(Utc::now());
        snapshot.total_files = total_files_atomic.load(Ordering::Relaxed);
        snapshot.total_bytes = total_bytes_atomic.load(Ordering::Relaxed);
        snapshot.deduped_bytes = deduped_bytes_atomic.load(Ordering::Relaxed);
        self.repository.update_snapshot(&snapshot)?;

        // --- SMART RETENTION: AUTO-PRUNING ---
        // Jika snapshot ini 100% identik dengan yang sebelumnya, kita hapus snapshot sebelumnya.
        // Ini memastikan timeline Anda hanya mencatat kejadian di mana ada perubahan data yang nyata.
        if let Some(prev) = latest_snapshot {
            if snapshot.total_bytes == prev.total_bytes
               && snapshot.deduped_bytes == snapshot.total_bytes
               && snapshot.total_files == prev.total_files {

                info!("Redundant snapshot detected (100% identical). Pruning previous snapshot: {}", prev.id.0);
                let _ = self.delete_snapshot(&prev.id);
            }
        }

        let default_strategy = domain::KeepCountStrategy { keep_limit: 10 };
        let _ = self.apply_retention_strategy(id, &default_strategy);

        Ok(snapshot)
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
        use sysinfo::Disks;
        let mut disks = Disks::new();
        disks.refresh_list();

        if let Some(disk) = disks.iter().next() {
            let available = disk.available_space();
            if available < required_bytes {
                anyhow::bail!(
                    "Insufficient disk space on host. Required: {:.2} MB, Available: {:.2} MB",
                    required_bytes as f64 / 1024.0 / 1024.0,
                    available as f64 / 1024.0 / 1024.0
                );
            }
            info!(
                "Storage check: OK (Available: {:.2} MB, Required max: {:.2} MB)",
                available as f64 / 1024.0 / 1024.0,
                required_bytes as f64 / 1024.0 / 1024.0
            );
        }
        Ok(())
    }

    pub(crate) fn backup_structured_data(
        &self,
        device_id: &DeviceId,
        snapshot_id: &domain::SnapshotId,
        encryption: &EncryptionMode,
    ) -> Result<()> {
        if let Ok(contacts) = self.data_provider.list_contacts(device_id) {
            let _ = self.store_structured_data(snapshot_id, "contacts", &contacts, encryption);

            // Also index contacts in database for global search
            for contact in contacts {
                if let Err(e) = self.repository.save_contact(snapshot_id, &contact) {
                    tracing::error!("Failed to index contact {}: {}", contact.display_name, e);
                }
            }
        }

        if let Ok(sms) = self.data_provider.list_sms(device_id) {
            let _ = self.store_structured_data(snapshot_id, "sms", &sms, encryption);
        }

        if let Ok(logs) = self.data_provider.list_call_logs(device_id) {
            let _ = self.store_structured_data(snapshot_id, "call_logs", &logs, encryption);
        }

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
