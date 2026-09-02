use crate::backup::BackupService;
use anyhow::Result;
use domain::{AppInfo, DeviceId};
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
    pub fn list_apps(&self, id: &DeviceId) -> Result<Vec<AppInfo>> {
        self.app_provider.list_apps(id)
    }

    #[instrument(skip(self))]
    pub fn export_apk(
        &self,
        device_id: &DeviceId,
        package_name: &str,
        target_path: &str,
    ) -> Result<()> {
        info!(
            "📦 Exporting APK for package '{}' on device {} -> {}",
            package_name, device_id, target_path
        );
        let mut apk_reader = self.app_provider.get_apk(device_id, package_name)?;
        let mut target_file = std::fs::File::create(target_path)?;
        std::io::copy(&mut apk_reader, &mut target_file)?;
        info!("✨ APK exported successfully: {}", target_path);
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn export_apk_batch(
        &self,
        device_id: &DeviceId,
        package_names: &[String],
        target_dir: &str,
    ) -> Result<Vec<String>> {
        info!(
            "📦 Exporting batch of {} APKs to directory: {}",
            package_names.len(),
            target_dir
        );
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

        info!(
            "✨ Batch APK export completed: {} succeeded",
            exported_files.len()
        );
        Ok(exported_files)
    }
}
