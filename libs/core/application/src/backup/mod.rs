pub mod builder;
pub mod guard;
pub mod manifest;
pub mod metadata;
pub mod planner;
pub mod processor;
pub mod progress;
pub mod restore;
pub mod scheduler;
pub mod service;
pub mod uploader;
pub mod verify;

pub use builder::BackupServiceBuilder;
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};
pub use verify::{StorageStats, VerificationReport};

/// The BackupService orchestrates use cases.
pub struct BackupService<
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
> {
    pub(crate) device_adapter: D,
    pub(crate) scanner_adapter: S,
    pub(crate) repository: R,
    pub storage: T,
    pub(crate) app_provider: A,
    pub(crate) data_provider: DP,
    pub(crate) progress: P,
}

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
    pub fn builder() -> BackupServiceBuilder<D, S, R, T, A, DP, P> {
        BackupServiceBuilder::new()
    }

    pub fn new(
        device_adapter: D,
        scanner_adapter: S,
        repository: R,
        storage: T,
        app_provider: A,
        data_provider: DP,
        progress: P,
    ) -> Self {
        Self {
            device_adapter,
            scanner_adapter,
            repository,
            storage,
            app_provider,
            data_provider,
            progress,
        }
    }
}
