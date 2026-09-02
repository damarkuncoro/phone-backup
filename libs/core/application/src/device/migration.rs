use crate::backup::BackupService;
use anyhow::Result;
use domain::DeviceId;
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};
use tracing::{info, instrument};

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
                    let _ = self
                        .device_adapter
                        .push_file(target_id, &mut *content, &file.path);
                }
            }
        }

        info!("✨ Migration completed!");
        Ok(())
    }
}
