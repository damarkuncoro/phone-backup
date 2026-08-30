pub mod client;
pub mod gateways;
pub mod parsers;
pub mod scripts;

// Re-export core types
pub use client::{AdbClient, AdbClientBuilder, AdbMonitor, DeviceEvent};
pub use gateways::{AdbGatewayFactory, AdbAdapter};

// Re-export concrete implementations for backward compatibility or direct use
pub use gateways::device::AdbDeviceGateway as AdbDeviceAdapter;
pub use gateways::app::AdbAppGateway as AdbAppProvider;
pub use gateways::scanner::AdbScannerGateway as AdbScannerAdapter;
pub use gateways::data::AdbDataGateway as AdbDataProvider;
