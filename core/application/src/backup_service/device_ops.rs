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
    pub fn scan_device(&self, id: &DeviceId) -> Result<Vec<FileEntry>> {
        self.scanner_adapter.scan(id, vec![])
    }

    #[instrument(skip(self))]
    pub fn list_apps(&self, id: &DeviceId) -> Result<Vec<AppInfo>> {
        self.app_provider.list_apps(id)
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
    pub fn get_structured_data(&self, snapshot_id: &SnapshotId, data_type: &str) -> Result<serde_json::Value> {
        tracing::info!("Fetching structured data '{}' for snapshot {}", data_type, snapshot_id.0);

        if data_type == "contacts" {
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
}
