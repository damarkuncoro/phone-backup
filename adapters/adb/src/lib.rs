pub mod client;
pub mod gateways;
pub mod parsers;
pub mod scripts;
pub mod repositories;

// Re-export core types
pub use client::{AdbClient, AdbClientBuilder, AdbMonitor, DeviceEvent};
pub use gateways::{AdbGatewayFactory, AdbAdapter};

// Exporting individual gateways for granular use if needed
pub use gateways::device::AdbDeviceGateway as AdbDeviceAdapter;
pub use gateways::app::AdbAppGateway as AdbAppProvider;
pub use gateways::scanner::AdbScannerGateway as AdbScannerAdapter;
pub use gateways::data::AdbDataGateway as AdbDataProvider;
