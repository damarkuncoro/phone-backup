//! Ports: the boundary traits that `application` depends on and that
//! `adapters` implement. This is the Dependency Inversion seam —
//! `application` never imports `adapter-*` crates directly.

mod app_provider_port;
mod data_provider_port;
pub mod decorators;
mod device_port;
mod progress_port;
mod repository_port;
mod scanner_port;
mod storage_port;

pub use app_provider_port::AppProviderPort;
pub use data_provider_port::DataProviderPort;
pub use decorators::{MetricsStorage, RetryStorage, StorageMetrics};
pub use device_port::DevicePort;
pub use progress_port::{NoProgress, ProgressPort};
pub use repository_port::{
    AppRepositoryPort, CallLogRepositoryPort, ContactRepositoryPort, DeviceRepositoryPort,
    FileRepositoryPort, MaintenanceRepositoryPort, RepositoryPort, ScheduleRepositoryPort,
    SettingsRepositoryPort, SmsRepositoryPort, SnapshotRepositoryPort,
};
pub use scanner_port::ScannerPort;
pub use storage_port::StoragePort;
