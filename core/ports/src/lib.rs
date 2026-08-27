//! Ports: the boundary traits that `application` depends on and that
//! `adapters` implement. This is the Dependency Inversion seam —
//! `application` never imports `adapter-*` crates directly.

mod device_port;
mod scanner_port;
mod repository_port;
mod storage_port;
mod app_provider_port;
mod data_provider_port;

pub use device_port::DevicePort;
pub use scanner_port::ScannerPort;
pub use repository_port::RepositoryPort;
pub use storage_port::StoragePort;
pub use app_provider_port::AppProviderPort;
pub use data_provider_port::DataProviderPort;
