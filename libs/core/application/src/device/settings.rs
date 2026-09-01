use crate::backup::BackupService;
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort, ProgressPort};
use domain::AppSettings;
use anyhow::Result;

impl<
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
> BackupService<D, S, R, T, A, DP, P>
{
    pub fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        self.repository.save_settings(settings)
    }

    pub fn get_settings(&self) -> Result<AppSettings> {
        Ok(self.repository.get_settings()?.unwrap_or_default())
    }
}
