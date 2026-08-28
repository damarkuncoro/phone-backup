use anyhow::Result;
use chrono::Utc;
use domain::{BackupPolicy, DeviceId, FileEntry, Snapshot, SnapshotStatus, EncryptionMode};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};
use std::io::Read;

use crate::compression::CompressionEngine;
use crate::hashing::calculate_hash;
use crate::media_analysis::MediaAnalyzer;
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
    > BackupService<D, S, R, T, A, DP>
{
    /// Perform a full or incremental backup of a device (Phase 07-21 + Storage Check + Resume + Asymmetric Crypto)
    #[instrument(skip(self, policy))]
    pub fn perform_backup(
        &self,
        id: &DeviceId,
        encryption: EncryptionMode,
        policy: Option<BackupPolicy>,
    ) -> Result<Snapshot> {
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

        let all_files = self.scanner_adapter.scan(id)?;
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

        use indicatif::{ProgressBar, ProgressStyle};
        use rayon::prelude::*;
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::sync::Mutex;

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
            .progress_chars("#>-"));

        let total_bytes_atomic = AtomicU64::new(snapshot.total_bytes);
        let total_files_atomic = AtomicU64::new(snapshot.total_files);
        let deduped_bytes_atomic = AtomicU64::new(snapshot.deduped_bytes);

        // We need a thread-safe way to update the snapshot if interrupted
        let snapshot_mutex = Mutex::new(snapshot);

        let result: Result<()> = files.into_par_iter().try_for_each(|mut file| {
            if already_backed_up.contains(&file.path) {
                pb.inc(1);
                return Ok(());
            }

            pb.set_message(format!("Processing {}", file.name));
            let mut skip_content = false;

            if let Some(prev) = previous_files.get(&file.path) {
                if prev.size_bytes == file.size_bytes
                    && prev.modified_at == file.modified_at
                    && prev.hash_sha256.is_some()
                {
                    file.hash_sha256 = prev.hash_sha256.clone();
                    skip_content = true;
                    deduped_bytes_atomic.fetch_add(file.size_bytes, Ordering::Relaxed);
                }
            }

            if !skip_content {
                match self.device_adapter.read_file(id, &file.path) {
                    Ok(mut content_reader) => {
                        let mut content_buf = Vec::new();
                        if let Err(e) = content_reader.read_to_end(&mut content_buf) {
                            let mut snap = snapshot_mutex.lock().unwrap();
                            self.mark_interrupted(&mut snap,
                                total_files_atomic.load(Ordering::Relaxed),
                                total_bytes_atomic.load(Ordering::Relaxed),
                                deduped_bytes_atomic.load(Ordering::Relaxed))?;
                            return Err(anyhow::anyhow!("Read error during backup: {}", e));
                        }

                        let hash = calculate_hash(&content_buf);
                        file.hash_sha256 = Some(hash.clone());
                        file.media_info = MediaAnalyzer::extract_info(&content_buf, &file.mime_type);

                        let object_id = ObjectStoreKey::compute_object_id(&hash, Some(&file.mime_type), encryption.is_encrypted());
                        let object_path = ObjectStoreKey::compute_object_path(&hash, &object_id);

                        if !self.storage.exists(&object_path)? {
                            let mut data_to_write = content_buf;
                            if CompressionEngine::should_compress(&file.mime_type) {
                                data_to_write = CompressionEngine::compress(&data_to_write)?;
                            }

                            data_to_write = match &encryption {
                                EncryptionMode::Password(pwd) => EncryptionEngine::encrypt(&data_to_write, pwd)?,
                                EncryptionMode::PublicKey(pk) => EncryptionEngine::encrypt_with_key(&data_to_write, pk)?,
                                EncryptionMode::None => data_to_write,
                            };

                            if let Err(e) = self.storage.write(&object_path, &mut std::io::Cursor::new(data_to_write)) {
                                let mut snap = snapshot_mutex.lock().unwrap();
                                self.mark_interrupted(&mut snap,
                                    total_files_atomic.load(Ordering::Relaxed),
                                    total_bytes_atomic.load(Ordering::Relaxed),
                                    deduped_bytes_atomic.load(Ordering::Relaxed))?;
                                return Err(anyhow::anyhow!("Storage write error: {}", e));
                            }
                        } else {
                            deduped_bytes_atomic.fetch_add(file.size_bytes, Ordering::Relaxed);
                        }
                    }
                    Err(e) => {
                        warn!("Warning: Failed to read file {}: {}", file.path, e);
                        pb.inc(1);
                        return Ok(());
                    }
                }
            }

            if let Err(e) = self.repository.save_file(&file) {
                let mut snap = snapshot_mutex.lock().unwrap();
                self.mark_interrupted(&mut snap,
                    total_files_atomic.load(Ordering::Relaxed),
                    total_bytes_atomic.load(Ordering::Relaxed),
                    deduped_bytes_atomic.load(Ordering::Relaxed))?;
                return Err(anyhow::anyhow!("Database error: {}", e));
            }
            let _ = self.repository.link_file_to_snapshot(&snapshot_mutex.lock().unwrap().id, &file.id);

            total_bytes_atomic.fetch_add(file.size_bytes, Ordering::Relaxed);
            total_files_atomic.fetch_add(1, Ordering::Relaxed);
            pb.inc(1);
            Ok(())
        });

        result?;

        pb.finish_with_message("File backup finished.");

        let mut snapshot = snapshot_mutex.into_inner().unwrap();

        if let Ok(apps) = self.app_provider.list_apps(id) {
            for app in apps {
                let _ = self.repository.save_app(&app);
                let _ = self.repository.link_app_to_snapshot(&snapshot.id, &app.id);
            }
        }

        let _ = self.backup_structured_data(id, &snapshot.id, &encryption);

        snapshot.status = SnapshotStatus::Completed;
        snapshot.finished_at = Some(Utc::now());
        snapshot.total_files = total_files_atomic.load(Ordering::Relaxed);
        snapshot.total_bytes = total_bytes_atomic.load(Ordering::Relaxed);
        snapshot.deduped_bytes = deduped_bytes_atomic.load(Ordering::Relaxed);
        self.repository.update_snapshot(&snapshot)?;

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
                    "Insufficient disk space on host. Required: {} GB, Available: {} GB",
                    required_bytes / 1024 / 1024 / 1024,
                    available / 1024 / 1024 / 1024
                );
            }
            info!(
                "Storage check: OK (Available: {} GB, Required max: {} GB)",
                available / 1024 / 1024 / 1024,
                required_bytes / 1024 / 1024 / 1024
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
        let contacts = self.data_provider.list_contacts(device_id)?;
        self.store_structured_data(snapshot_id, "contacts", &contacts, encryption)?;

        let sms = self.data_provider.list_sms(device_id)?;
        self.store_structured_data(snapshot_id, "sms", &sms, encryption)?;

        let logs = self.data_provider.list_call_logs(device_id)?;
        self.store_structured_data(snapshot_id, "call_logs", &logs, encryption)?;

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
