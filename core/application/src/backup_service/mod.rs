pub mod backup;
pub mod device_ops;
pub mod processor;
pub mod restore;
pub mod uploader;
pub mod metadata;
pub mod schedule_runner;
pub mod verify;
pub mod settings_ops;

pub use verify::{StorageStats, VerificationReport};

use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort, ProgressPort};

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
