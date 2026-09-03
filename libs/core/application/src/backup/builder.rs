use super::BackupService;
use anyhow::{bail, Result};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};

/// Builder for fluently creating instances of `BackupService`.
pub struct BackupServiceBuilder<D, S, R, T, A, DP, P> {
    device_adapter: Option<D>,
    scanner_adapter: Option<S>,
    repository: Option<R>,
    storage: Option<T>,
    app_provider: Option<A>,
    data_provider: Option<DP>,
    progress: Option<P>,
}

impl<D, S, R, T, A, DP, P> Default for BackupServiceBuilder<D, S, R, T, A, DP, P> {
    fn default() -> Self {
        Self {
            device_adapter: None,
            scanner_adapter: None,
            repository: None,
            storage: None,
            app_provider: None,
            data_provider: None,
            progress: None,
        }
    }
}

impl<D, S, R, T, A, DP, P> BackupServiceBuilder<D, S, R, T, A, DP, P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_device_adapter(mut self, device: D) -> Self {
        self.device_adapter = Some(device);
        self
    }

    pub fn with_scanner_adapter(mut self, scanner: S) -> Self {
        self.scanner_adapter = Some(scanner);
        self
    }

    pub fn with_repository(mut self, repository: R) -> Self {
        self.repository = Some(repository);
        self
    }

    pub fn with_storage(mut self, storage: T) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn with_app_provider(mut self, app_provider: A) -> Self {
        self.app_provider = Some(app_provider);
        self
    }

    pub fn with_data_provider(mut self, data_provider: DP) -> Self {
        self.data_provider = Some(data_provider);
        self
    }

    pub fn with_progress(mut self, progress: P) -> Self {
        self.progress = Some(progress);
        self
    }
}

impl<D, S, R, T, A, DP, P> BackupServiceBuilder<D, S, R, T, A, DP, P>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
{
    /// Build the `BackupService` instance, validating that all required ports are present.
    pub fn build(self) -> Result<BackupService<D, S, R, T, A, DP, P>> {
        let device_adapter = match self.device_adapter {
            Some(d) => d,
            None => bail!("Missing required device_adapter in BackupServiceBuilder"),
        };
        let scanner_adapter = match self.scanner_adapter {
            Some(s) => s,
            None => bail!("Missing required scanner_adapter in BackupServiceBuilder"),
        };
        let repository = match self.repository {
            Some(r) => r,
            None => bail!("Missing required repository in BackupServiceBuilder"),
        };
        let storage = match self.storage {
            Some(t) => t,
            None => bail!("Missing required storage in BackupServiceBuilder"),
        };
        let app_provider = match self.app_provider {
            Some(a) => a,
            None => bail!("Missing required app_provider in BackupServiceBuilder"),
        };
        let data_provider = match self.data_provider {
            Some(dp) => dp,
            None => bail!("Missing required data_provider in BackupServiceBuilder"),
        };
        let progress = match self.progress {
            Some(p) => p,
            None => bail!("Missing required progress port in BackupServiceBuilder"),
        };

        Ok(BackupService::new(
            device_adapter,
            scanner_adapter,
            repository,
            storage,
            app_provider,
            data_provider,
            progress,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adapter_database_sqlite::SqliteRepository;
    use adapter_filesystem::LocalStorage;
    use adapter_mock::{MockAppProvider, MockDataProvider, MockDeviceAdapter, MockScannerAdapter};
    use tempfile::TempDir;

    #[test]
    fn test_backup_service_builder() {
        let tmp_dir = TempDir::new().unwrap();
        let db_path = tmp_dir.path().join("test_builder.db");
        let storage_path = tmp_dir.path().to_str().unwrap();

        let repo = SqliteRepository::new(db_path.to_str().unwrap()).unwrap();
        let storage = LocalStorage::new(storage_path).unwrap();

        let service = BackupService::builder()
            .with_device_adapter(MockDeviceAdapter::default())
            .with_scanner_adapter(MockScannerAdapter)
            .with_repository(repo)
            .with_storage(storage)
            .with_app_provider(MockAppProvider)
            .with_data_provider(MockDataProvider)
            .with_progress(ports::NoProgress)
            .build()
            .expect("BackupServiceBuilder should successfully construct BackupService");

        let devices = service.list_devices().unwrap();
        assert_eq!(devices.len(), 1);
    }
}
