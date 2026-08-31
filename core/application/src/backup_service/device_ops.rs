use anyhow::Result;
use domain::{AppInfo, CapabilityMatrix, Device, DeviceId, FileEntry, Snapshot, SnapshotId};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};

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
    #[instrument(skip(self))]
    pub fn list_devices(&self) -> Result<Vec<Device>> {
        self.device_adapter.discover()
    }

    #[instrument(skip(self))]
    pub fn list_all_known_devices(&self) -> Result<Vec<Device>> {
        self.repository.list_devices()
    }

    #[instrument(skip(self))]
    pub fn device_info(&self, id: &DeviceId) -> Result<Device> {
        self.device_adapter.info(id)
    }

    #[instrument(skip(self))]
    pub fn device_capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        self.device_adapter.capabilities(id)
    }

    #[instrument(skip(self))]
    pub fn get_device_battery(&self, id: &DeviceId) -> Result<(u32, f32)> {
        self.device_adapter.battery_status(id)
    }

    #[instrument(skip(self))]
    pub fn list_directory(&self, id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        self.device_adapter.list_directory(id, path)
    }

    #[instrument(skip(self))]
    pub fn delete_remote(&self, id: &DeviceId, path: &str) -> Result<()> {
        self.device_adapter.delete_remote(id, path)
    }

    #[instrument(skip(self))]
    pub fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()> {
        self.device_adapter.rename_remote(id, old_path, new_path)
    }

    #[instrument(skip(self))]
    pub fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        self.device_adapter.copy_remote(id, source_path, target_path)
    }

    #[instrument(skip(self))]
    pub fn calculate_hash(&self, id: &DeviceId, path: &str) -> Result<String> {
        self.device_adapter.calculate_hash(id, path)
    }

    #[instrument(skip(self))]
    pub fn upload_file(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        let mut file = std::fs::File::open(source_path)?;
        self.device_adapter.push_file(id, &mut file, target_path)
    }

    #[instrument(skip(self))]
    pub fn download_file(&self, id: &DeviceId, remote_path: &str, local_path: &str) -> Result<()> {
        let mut reader = self.device_adapter.read_file(id, remote_path)?;
        let mut file = std::fs::File::create(local_path)?;
        std::io::copy(&mut reader, &mut file)?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn scan_device(&self, id: &DeviceId) -> Result<Vec<FileEntry>> {
        self.scanner_adapter.scan(id, vec![])
    }

    #[instrument(skip(self))]
    pub fn list_apps(&self, id: &DeviceId) -> Result<Vec<AppInfo>> {
        self.app_provider.list_apps(id)
    }

    #[instrument(skip(self))]
    pub fn export_apk(&self, device_id: &DeviceId, package_name: &str, target_path: &str) -> Result<()> {
        info!("📦 Exporting APK for package '{}' on device {} -> {}", package_name, device_id, target_path);
        let mut apk_reader = self.app_provider.get_apk(device_id, package_name)?;
        let mut target_file = std::fs::File::create(target_path)?;
        std::io::copy(&mut apk_reader, &mut target_file)?;
        info!("✨ APK exported successfully: {}", target_path);
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn export_apk_batch(&self, device_id: &DeviceId, package_names: &[String], target_dir: &str) -> Result<Vec<String>> {
        info!("📦 Exporting batch of {} APKs to directory: {}", package_names.len(), target_dir);
        std::fs::create_dir_all(target_dir)?;
        let mut exported_files = Vec::new();

        for pkg in package_names {
            let filename = format!("{}.apk", pkg);
            let target_path = std::path::Path::new(target_dir).join(&filename);
            let target_path_str = target_path.to_str().unwrap_or(&filename);

            if let Ok(()) = self.export_apk(device_id, pkg, target_path_str) {
                exported_files.push(target_path_str.to_string());
            }
        }

        info!("✨ Batch APK export completed: {} succeeded", exported_files.len());
        Ok(exported_files)
    }

    #[instrument(skip(self))]
    pub fn list_contacts(&self, id: &DeviceId) -> Result<Vec<domain::Contact>> {
        self.data_provider.list_contacts(id)
    }

    #[instrument(skip(self))]
    pub fn list_sms(&self, id: &DeviceId) -> Result<Vec<domain::Sms>> {
        self.data_provider.list_sms(id)
    }

    #[instrument(skip(self))]
    pub fn list_call_logs(&self, id: &DeviceId) -> Result<Vec<domain::CallLog>> {
        self.data_provider.list_call_logs(id)
    }

    #[instrument(skip(self))]
    pub fn list_snapshots(&self, id: &DeviceId) -> Result<Vec<Snapshot>> {
        self.repository.list_snapshots(id)
    }

    #[instrument(skip(self))]
    pub fn get_latest_snapshot_any_device(&self) -> Result<Option<Snapshot>> {
        let devices = self.list_devices()?;
        let mut latest: Option<Snapshot> = None;
        for d in devices {
            if let Ok(snapshots) = self.list_snapshots(&d.id) {
                if let Some(s) = snapshots.into_iter().find(|s| s.status == domain::SnapshotStatus::Completed) {
                    if latest.is_none() || s.started_at > latest.as_ref().unwrap().started_at {
                        latest = Some(s);
                    }
                }
            }
        }
        Ok(latest)
    }

    #[instrument(skip(self))]
    pub fn get_snapshot(&self, id: &SnapshotId) -> Result<Option<Snapshot>> {
        self.repository.get_snapshot(id)
    }

    #[instrument(skip(self))]
    pub fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> Result<Vec<AppInfo>> {
        self.repository.get_snapshot_apps(snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> Result<Vec<FileEntry>> {
        self.repository.get_snapshot_files(snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn get_structured_data(&self, snapshot_id: &SnapshotId, data_type: domain::StructuredDataType) -> Result<serde_json::Value> {
        tracing::info!("Fetching structured data '{}' for snapshot {}", data_type, snapshot_id.0);

        if data_type == domain::StructuredDataType::Contacts {
            let contacts = self.repository.get_snapshot_contacts(snapshot_id)?;
            return Ok(serde_json::to_value(contacts)?);
        }

        let object_path = self.repository.get_structured_data_ref(snapshot_id, data_type)?
            .ok_or_else(|| {
                tracing::warn!("Structured data '{}' reference not found in database", data_type);
                anyhow::anyhow!("Data type {} not found for this snapshot", data_type)
            })?;

        tracing::info!("Reading data from storage: {}", object_path);
        let mut reader = self.storage.read(&object_path)?;
        let mut data = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut data)?;

        tracing::info!("Parsing JSON data ({} bytes)", data.len());
        let json: serde_json::Value = serde_json::from_slice(&data).map_err(|e| {
            tracing::error!("JSON parse error: {}. Data might be encrypted.", e);
            e
        })?;
        Ok(json)
    }

    #[instrument(skip(self))]
    pub fn delete_snapshot(&self, id: &SnapshotId) -> Result<()> {
        self.repository.delete_snapshot(id)
    }

    #[instrument(skip(self))]
    pub fn prune_failed_snapshots(&self) -> Result<usize> {
        let snapshots = self.repository.list_all_snapshots()?;
        let mut deleted_count = 0;

        for s in snapshots {
            if s.status != domain::SnapshotStatus::Completed {
                info!("Pruning incomplete/failed snapshot: {} (status: {:?})", s.id.0, s.status);
                self.delete_snapshot(&s.id)?;
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }

    #[instrument(skip(self))]
    pub fn search_files(&self, query: &str) -> Result<Vec<FileEntry>> {
        self.repository.search_files(query)
    }

    #[instrument(skip(self))]
    pub fn search_contacts(&self, query: &str) -> Result<Vec<(SnapshotId, domain::Contact)>> {
        self.repository.search_contacts(query)
    }

    #[instrument(skip(self))]
    pub fn search_sms(&self, query: &str) -> Result<Vec<(SnapshotId, domain::Sms)>> {
        self.repository.search_sms(query)
    }

    #[instrument(skip(self))]
    pub fn list_media_files(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        self.repository.list_media_files(device_id)
    }

    #[instrument(skip(self))]
    pub fn get_file_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> Result<domain::FileDiff> {
        self.repository.get_file_diff(old_snapshot_id, new_snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn migrate_device(&self, source_id: &DeviceId, target_id: &DeviceId) -> Result<()> {
        info!("🚀 Starting migration: {} -> {}", source_id, target_id);

        if let Ok(apps) = self.app_provider.list_apps(source_id) {
            for app in apps {
                info!("   Installing {}...", app.app_name);
                if let Ok(mut apk) = self.app_provider.get_apk(source_id, &app.package_name) {
                    let _ = self.app_provider.install_app(target_id, &mut *apk);
                }
            }
        }

        if let Ok(files) = self.scanner_adapter.scan(source_id, vec![]) {
            for file in files {
                if let Ok(mut content) = self.device_adapter.read_file(source_id, &file.path) {
                    let _ = self.device_adapter.push_file(target_id, &mut *content, &file.path);
                }
            }
        }

        info!("✨ Migration completed!");
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn get_contact_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> Result<domain::ContactDiff> {
        self.repository.get_contact_diff(old_snapshot_id, new_snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn export_contacts_vcard(&self, snapshot_id: &SnapshotId) -> Result<String> {
        let contacts = self.repository.get_snapshot_contacts(snapshot_id)?;
        Ok(crate::VCardEngine::export_to_vcard(&contacts))
    }

    #[instrument(skip(self))]
    pub fn get_snapshot_sms(&self, snapshot_id: &SnapshotId) -> Result<Vec<domain::Sms>> {
        self.repository.get_snapshot_sms(snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn export_sms_json(&self, snapshot_id: &SnapshotId) -> Result<String> {
        let sms_list = self.repository.get_snapshot_sms(snapshot_id)?;
        Ok(serde_json::to_string_pretty(&sms_list)?)
    }

    #[instrument(skip(self))]
    pub fn get_snapshot_call_logs(&self, snapshot_id: &SnapshotId) -> Result<Vec<domain::CallLog>> {
        self.repository.get_snapshot_call_logs(snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn search_call_logs(&self, query: &str) -> Result<Vec<(SnapshotId, domain::CallLog)>> {
        self.repository.search_call_logs(query)
    }

    #[instrument(skip(self))]
    pub fn export_call_logs_json(&self, snapshot_id: &SnapshotId) -> Result<String> {
        let logs = self.repository.get_snapshot_call_logs(snapshot_id)?;
        Ok(serde_json::to_string_pretty(&logs)?)
    }
}
