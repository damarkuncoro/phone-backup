use anyhow::Result;
use domain::{CapabilityMatrix, Device, DeviceId, FileEntry, Snapshot, SnapshotId, SnapshotStatus, AppInfo, BackupSchedule, ScheduleFrequency, BackupPolicy, RetentionPolicy};
use ports::{DevicePort, ScannerPort, RepositoryPort, StoragePort, AppProviderPort, DataProviderPort};
use chrono::Utc;
use std::io::Read;

use crate::security::EncryptionEngine;
use crate::media_analysis::MediaAnalyzer;
use crate::compression::CompressionEngine;
use crate::hashing::calculate_hash;

/// The BackupService orchestrates use cases.
pub struct BackupService<D: DevicePort, S: ScannerPort, R: RepositoryPort, T: StoragePort, A: AppProviderPort, DP: DataProviderPort> {
    device_adapter: D,
    scanner_adapter: S,
    repository: R,
    storage: T,
    app_provider: A,
    data_provider: DP,
}

impl<D: DevicePort, S: ScannerPort, R: RepositoryPort, T: StoragePort, A: AppProviderPort, DP: DataProviderPort> BackupService<D, S, R, T, A, DP> {
    pub fn new(device_adapter: D, scanner_adapter: S, repository: R, storage: T, app_provider: A, data_provider: DP) -> Self {
        Self {
            device_adapter,
            scanner_adapter,
            repository,
            storage,
            app_provider,
            data_provider,
        }
    }

    pub fn list_devices(&self) -> Result<Vec<Device>> {
        self.device_adapter.discover()
    }

    pub fn device_info(&self, id: &DeviceId) -> Result<Device> {
        self.device_adapter.info(id)
    }

    pub fn device_capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        self.device_adapter.capabilities(id)
    }

    pub fn scan_device(&self, id: &DeviceId) -> Result<Vec<FileEntry>> {
        self.scanner_adapter.scan(id)
    }

    pub fn list_apps(&self, id: &DeviceId) -> Result<Vec<AppInfo>> {
        self.app_provider.list_apps(id)
    }

    /// Perform a full or incremental backup of a device (Phase 07-21 + Storage Check + Resume)
    pub fn perform_backup(&self, id: &DeviceId, password: Option<&str>, policy: Option<BackupPolicy>) -> Result<Snapshot> {
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
            println!("🔄 Resuming interrupted snapshot: {}", incomplete.id.0);
            incomplete
        } else {
            Snapshot::new(id.clone())
        };

        let already_backed_up: std::collections::HashSet<String> = self.repository
            .get_snapshot_files(&snapshot.id)?
            .into_iter()
            .map(|f| f.path)
            .collect();

        snapshot.status = SnapshotStatus::Running;
        self.repository.create_snapshot(&snapshot).or_else(|_| self.repository.update_snapshot(&snapshot))?;

        let all_files = self.scanner_adapter.scan(id)?;
        let files: Vec<FileEntry> = all_files.into_iter()
            .filter(|f| policy.should_include(&f.path))
            .collect();

        let total_required: u64 = files.iter()
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
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
            .progress_chars("#>-"));

        let mut total_bytes = snapshot.total_bytes;
        let mut total_files = snapshot.total_files;
        let mut deduped_bytes = snapshot.deduped_bytes;

        for mut file in files {
            if already_backed_up.contains(&file.path) {
                pb.inc(1);
                continue;
            }

            pb.set_message(format!("Processing {}", file.name));
            let mut skip_content = false;

            if let Some(prev) = previous_files.get(&file.path) {
                if prev.size_bytes == file.size_bytes &&
                   prev.modified_at == file.modified_at &&
                   prev.hash_sha256.is_some()
                {
                    file.hash_sha256 = prev.hash_sha256.clone();
                    skip_content = true;
                    deduped_bytes += file.size_bytes;
                }
            }

            if !skip_content {
                match self.device_adapter.read_file(id, &file.path) {
                    Ok(mut content_reader) => {
                        let mut content_buf = Vec::new();
                        if let Err(e) = content_reader.read_to_end(&mut content_buf) {
                            self.mark_interrupted(&mut snapshot, total_files, total_bytes, deduped_bytes)?;
                            return Err(anyhow::anyhow!("Read error during backup: {}", e));
                        }

                        let hash = calculate_hash(&content_buf);
                        file.hash_sha256 = Some(hash.clone());
                        file.media_info = MediaAnalyzer::extract_info(&content_buf, &file.mime_type);

                        let mut object_id = if CompressionEngine::should_compress(&file.mime_type) {
                            format!("{}.zst", hash)
                        } else {
                            hash.clone()
                        };

                        if password.is_some() {
                            object_id = format!("{}.enc", object_id);
                        }

                        let object_path = format!("objects/{}/{}/{}", &hash[0..2], &hash[2..4], object_id);

                        if !self.storage.exists(&object_path)? {
                            let mut data_to_write = content_buf;
                            if CompressionEngine::should_compress(&file.mime_type) {
                                data_to_write = CompressionEngine::compress(&data_to_write)?;
                            }
                            if let Some(pwd) = password {
                                data_to_write = EncryptionEngine::encrypt(&data_to_write, pwd)?;
                            }
                            if let Err(e) = self.storage.write(&object_path, &mut std::io::Cursor::new(data_to_write)) {
                                self.mark_interrupted(&mut snapshot, total_files, total_bytes, deduped_bytes)?;
                                return Err(anyhow::anyhow!("Storage write error: {}", e));
                            }
                        } else {
                            deduped_bytes += file.size_bytes;
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to read file {}: {}", file.path, e);
                        pb.inc(1);
                        continue;
                    }
                }
            }

            if let Err(e) = self.repository.save_file(&file) {
                self.mark_interrupted(&mut snapshot, total_files, total_bytes, deduped_bytes)?;
                return Err(anyhow::anyhow!("Database error: {}", e));
            }
            let _ = self.repository.link_file_to_snapshot(&snapshot.id, &file.id);

            total_bytes += file.size_bytes;
            total_files += 1;
            pb.inc(1);
        }
        pb.finish_with_message("File backup finished.");

        if let Ok(apps) = self.app_provider.list_apps(id) {
            for app in apps {
                let _ = self.repository.save_app(&app);
                let _ = self.repository.link_app_to_snapshot(&snapshot.id, &app.id);
            }
        }

        let _ = self.backup_structured_data(id, &snapshot.id, password);

        snapshot.status = SnapshotStatus::Completed;
        snapshot.finished_at = Some(Utc::now());
        snapshot.total_files = total_files;
        snapshot.total_bytes = total_bytes;
        snapshot.deduped_bytes = deduped_bytes;
        self.repository.update_snapshot(&snapshot)?;

        let _ = self.apply_retention_policy(id, domain::RetentionPolicy::default());

        Ok(snapshot)
    }

    fn mark_interrupted(&self, snapshot: &mut Snapshot, files: u64, bytes: u64, dedup: u64) -> Result<()> {
        snapshot.status = SnapshotStatus::Interrupted;
        snapshot.total_files = files;
        snapshot.total_bytes = bytes;
        snapshot.deduped_bytes = dedup;
        self.repository.update_snapshot(snapshot)?;
        Ok(())
    }

    fn check_available_disk_space(&self, required_bytes: u64) -> Result<()> {
        use sysinfo::{Disks};
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
            println!("Storage check: OK (Available: {} GB, Required max: {} GB)",
                available / 1024 / 1024 / 1024,
                required_bytes / 1024 / 1024 / 1024
            );
        }
        Ok(())
    }

    pub fn list_snapshots(&self, id: &DeviceId) -> Result<Vec<Snapshot>> {
        self.repository.list_snapshots(id)
    }

    pub fn get_snapshot(&self, id: &SnapshotId) -> Result<Option<Snapshot>> {
        self.repository.get_snapshot(id)
    }

    pub fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> Result<Vec<AppInfo>> {
        self.repository.get_snapshot_apps(snapshot_id)
    }

    pub fn delete_snapshot(&self, id: &SnapshotId) -> Result<()> {
        self.repository.delete_snapshot(id)
    }

    pub fn apply_retention_policy(&self, device_id: &DeviceId, policy: RetentionPolicy) -> Result<u32> {
        let snapshots = self.repository.list_snapshots(device_id)?;
        let mut completed_snapshots: Vec<_> = snapshots.into_iter()
            .filter(|s| s.status == SnapshotStatus::Completed)
            .collect();

        completed_snapshots.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        let mut deleted_count = 0;
        let limit = policy.keep_daily as usize;

        if completed_snapshots.len() > limit {
            for s in completed_snapshots.iter().skip(limit) {
                println!("Auto-cleanup: Deleting old snapshot {} (Retention)", s.id.0);
                self.repository.delete_snapshot(&s.id)?;
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }

    pub fn add_schedule(&self, device_id: DeviceId, frequency: ScheduleFrequency) -> Result<()> {
        let schedule = BackupSchedule {
            device_id,
            frequency,
            last_run_at: None,
            enabled: true,
        };
        self.repository.save_schedule(&schedule)
    }

    pub fn list_schedules(&self) -> Result<Vec<BackupSchedule>> {
        self.repository.list_schedules()
    }

    pub fn run_pending_backups(&self, password: Option<&str>) -> Result<()> {
        let schedules = self.repository.list_schedules()?;
        let connected_devices = self.device_adapter.discover()?;

        for schedule in schedules {
            if schedule.is_due() {
                if connected_devices.iter().any(|d| d.id == schedule.device_id) {
                    println!("Running scheduled backup for device {}...", schedule.device_id);
                    match self.perform_backup(&schedule.device_id, password, None) {
                        Ok(_) => {
                            let mut updated_schedule = schedule;
                            updated_schedule.last_run_at = Some(Utc::now());
                            self.repository.save_schedule(&updated_schedule)?;
                        }
                        Err(e) => {
                            eprintln!("Scheduled backup failed for {}: {}", schedule.device_id, e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn perform_restore(&self, snapshot_id: &SnapshotId, target_dir: &str, password: Option<&str>, filter: Option<&str>) -> Result<()> {
        use std::fs;
        use std::path::Path;

        let files = self.repository.get_snapshot_files(snapshot_id)?;
        let target_base = Path::new(target_dir);

        for file in files {
            if let Some(f) = filter {
                if !file.path.contains(f) && !file.name.contains(f) {
                    continue;
                }
            }

            let hash = file.hash_sha256.as_ref().ok_or_else(|| anyhow::anyhow!("File {} has no hash", file.path))?;

            let mut object_id = if CompressionEngine::should_compress(&file.mime_type) {
                format!("{}.zst", hash)
            } else {
                hash.clone()
            };

            if password.is_some() {
                object_id = format!("{}.enc", object_id);
            }

            let object_path = format!("objects/{}/{}/{}", &hash[0..2], &hash[2..4], object_id);

            let mut reader = self.storage.read(&object_path)?;
            let mut data = Vec::new();
            reader.read_to_end(&mut data)?;

            if object_id.ends_with(".enc") {
                let pwd = password.ok_or_else(|| anyhow::anyhow!("Password required for encrypted backup"))?;
                data = EncryptionEngine::decrypt(&data, pwd)?;
            }

            if object_id.contains(".zst") {
                data = CompressionEngine::decompress(&data)?;
            }

            let restore_path = target_base.join(&file.path);
            if let Some(parent) = restore_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(restore_path, data)?;
        }

        Ok(())
    }

    pub fn verify_repository(&self, password: Option<&str>) -> Result<VerificationReport> {
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

                let mut object_id = if CompressionEngine::should_compress(&file.mime_type) {
                    format!("{}.zst", hash)
                } else {
                    hash.clone()
                };

                if password.is_some() {
                    object_id = format!("{}.enc", object_id);
                }

                let object_path = format!("objects/{}/{}/{}", &hash[0..2], &hash[2..4], object_id);

                if !self.storage.exists(&object_path)? {
                    report.missing_objects.push(file.path);
                    continue;
                }

                report.verified_files += 1;
            }
        }

        Ok(report)
    }

    pub fn search_files(&self, query: &str) -> Result<Vec<FileEntry>> {
        self.repository.search_files(query)
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

    pub fn migrate_device(&self, source_id: &DeviceId, target_id: &DeviceId) -> Result<()> {
        println!("🚀 Starting migration: {} -> {}", source_id, target_id);

        if let Ok(apps) = self.app_provider.list_apps(source_id) {
            for app in apps {
                println!("   Installing {}...", app.app_name);
                if let Ok(mut apk) = self.app_provider.get_apk(source_id, &app.package_name) {
                    let _ = self.app_provider.install_app(target_id, &mut *apk);
                }
            }
        }

        if let Ok(files) = self.scanner_adapter.scan(source_id) {
            use indicatif::{ProgressBar, ProgressStyle};
            let pb = ProgressBar::new(files.len() as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
                .progress_chars("#>-"));

            for file in files {
                pb.set_message(format!("Transferring {}", file.name));
                if let Ok(mut content) = self.device_adapter.read_file(source_id, &file.path) {
                    let _ = self.device_adapter.push_file(target_id, &mut *content, &file.path);
                }
                pb.inc(1);
            }
            pb.finish_with_message("Files migrated.");
        }

        println!("✨ Migration completed!");
        Ok(())
    }

    fn backup_structured_data(&self, device_id: &DeviceId, snapshot_id: &SnapshotId, password: Option<&str>) -> Result<()> {
        let contacts = self.data_provider.list_contacts(device_id)?;
        self.store_structured_data(snapshot_id, "contacts", &contacts, password)?;

        let sms = self.data_provider.list_sms(device_id)?;
        self.store_structured_data(snapshot_id, "sms", &sms, password)?;

        let logs = self.data_provider.list_call_logs(device_id)?;
        self.store_structured_data(snapshot_id, "call_logs", &logs, password)?;

        Ok(())
    }

    fn store_structured_data<V: serde::Serialize>(&self, snapshot_id: &SnapshotId, data_type: &str, data: &V, password: Option<&str>) -> Result<()> {
        let json = serde_json::to_vec(data)?;
        let hash = calculate_hash(&json);

        let mut object_id = format!("{}.json", hash);
        if password.is_some() {
            object_id = format!("{}.enc", object_id);
        }

        let object_path = format!("objects/{}/{}/{}", &hash[0..2], &hash[2..4], object_id);

        if !self.storage.exists(&object_path)? {
            let mut data_to_write = json;
            if let Some(pwd) = password {
                data_to_write = EncryptionEngine::encrypt(&data_to_write, pwd)?;
            }
            self.storage.write(&object_path, &mut std::io::Cursor::new(data_to_write))?;
        }

        self.repository.save_structured_data_ref(snapshot_id, data_type, &object_path)?;
        Ok(())
    }
}

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
        if self.total_logical_bytes == 0 { return 1.0; }
        (self.total_deduped_bytes as f64 / self.total_logical_bytes as f64) * 100.0
    }
}
