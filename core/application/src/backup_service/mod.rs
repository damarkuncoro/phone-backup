pub mod backup;
pub mod device_ops;
pub mod processor;
pub mod restore;
pub mod schedule_runner;
pub mod verify;

pub use verify::{StorageStats, VerificationReport};

use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};

/// The BackupService orchestrates use cases.
pub struct BackupService<
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
> {
    pub(crate) device_adapter: D,
    pub(crate) scanner_adapter: S,
    pub(crate) repository: R,
    pub(crate) storage: T,
    pub(crate) app_provider: A,
    pub(crate) data_provider: DP,
}

impl<
        D: DevicePort,
        S: ScannerPort,
        R: RepositoryPort,
        T: StoragePort,
        A: AppProviderPort,
        DP: DataProviderPort,
    > BackupService<D, S, R, T, A, DP>
{
    pub fn new(
        device_adapter: D,
        scanner_adapter: S,
        repository: R,
        storage: T,
        app_provider: A,
        data_provider: DP,
    ) -> Self {
        Self {
            device_adapter,
            scanner_adapter,
            repository,
            storage,
            app_provider,
            data_provider,
        }
    }
}
