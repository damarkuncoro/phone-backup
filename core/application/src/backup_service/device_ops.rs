use anyhow::Result;
use domain::{AppInfo, CapabilityMatrix, Device, DeviceId, FileEntry, Snapshot, SnapshotId};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};

use tracing::info;

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
        self.scanner_adapter.scan(id, vec![])
    }

    pub fn list_apps(&self, id: &DeviceId) -> Result<Vec<AppInfo>> {
        self.app_provider.list_apps(id)
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

    pub fn search_files(&self, query: &str) -> Result<Vec<FileEntry>> {
        self.repository.search_files(query)
    }

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
