pub mod client;
pub mod gateways;
pub mod parsers;
pub mod scripts;
pub mod repositories;
pub mod scanner;

// Re-export core types
pub use client::{AdbClient, AdbClientBuilder, AdbMonitor, DeviceEvent};
pub use gateways::{AdbGatewayFactory, AdbAdapter};
pub use scanner::{ScannerAggregator, MediaStoreScanner, FileSystemScanner};

// Exporting individual gateways for granular use if needed
pub use gateways::device::AdbDeviceGateway as AdbDeviceAdapter;
pub use gateways::app::AdbAppGateway as AdbAppProvider;
pub use gateways::scanner::AdbScannerGateway as AdbScannerAdapter;
pub use gateways::data::AdbDataGateway as AdbDataProvider;
