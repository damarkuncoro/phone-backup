pub mod device;
pub mod app;
pub mod scanner;
pub mod data;

pub use device::AdbDeviceGateway;
pub use app::AdbAppGateway;
pub use scanner::AdbScannerGateway;
pub use data::AdbDataGateway;

use crate::client::AdbClient;
use std::sync::Arc;

/// FACTORY Pattern: Centralizes the creation of ADB-related gateways.
pub struct AdbGatewayFactory {
    client: AdbClient,
}

impl AdbGatewayFactory {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    pub fn create_device_gateway(&self) -> AdbDeviceGateway {
        AdbDeviceGateway::new(self.client.clone())
    }

    pub fn create_app_gateway(&self) -> AdbAppGateway {
        AdbAppGateway::new(self.client.clone())
    }

    pub fn create_scanner_gateway(&self) -> AdbScannerGateway {
        AdbScannerGateway::new(self.client.clone())
    }

    pub fn create_data_gateway(&self) -> AdbDataGateway {
        AdbDataGateway::new(self.client.clone())
    }

    /// Creates an aggregate adapter (Facade) containing all gateways.
    pub fn create_all(self) -> Arc<AdbAdapter> {
        Arc::new(AdbAdapter {
            client: self.client.clone(),
            device: self.create_device_gateway(),
            app: self.create_app_gateway(),
            scanner: self.create_scanner_gateway(),
            data: self.create_data_gateway(),
        })
    }
}

/// FACADE Pattern: A unified interface that aggregates all ADB-specific gateways.
pub struct AdbAdapter {
    pub client: AdbClient,
    pub device: AdbDeviceGateway,
    pub app: AdbAppGateway,
    pub scanner: AdbScannerGateway,
    pub data: AdbDataGateway,
}
