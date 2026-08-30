pub mod client;
pub mod gateways;
pub mod parsers;
pub mod scripts;

// Re-export concrete implementations for use by the application layer
pub use gateways::device::AdbDeviceGateway as AdbDeviceAdapter;
pub use gateways::app::AdbAppGateway as AdbAppProvider;
pub use gateways::scanner::AdbScannerGateway as AdbScannerAdapter;
pub use gateways::data::AdbDataGateway as AdbDataProvider;
pub use client::AdbClient;
