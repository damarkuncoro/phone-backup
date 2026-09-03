pub mod client;
pub mod gateways;
pub mod parsers;
pub mod pool;
pub mod repositories;
pub mod scanner;
pub mod scripts;

// Re-export core types
pub use client::{AdbClient, AdbClientBuilder, AdbMonitor, DeviceEvent};
pub use gateways::{AdbAdapter, AdbGatewayFactory};
pub use pool::{AdbWorkerPool, ConcurrentAdbStreamer};
pub use scanner::{FileSystemScanner, MediaStoreScanner, ScannerAggregator};

// Exporting individual gateways for granular use if needed
pub use gateways::app::AdbAppGateway as AdbAppProvider;
pub use gateways::data::AdbDataGateway as AdbDataProvider;
pub use gateways::device::AdbDeviceGateway as AdbDeviceAdapter;
pub use gateways::scanner::AdbScannerGateway as AdbScannerAdapter;
