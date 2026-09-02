pub mod app;
pub mod data;
pub mod device;
pub mod facade;
pub mod scanner;

pub use app::AdbAppGateway;
pub use data::AdbDataGateway;
pub use device::AdbDeviceGateway;
pub use facade::AdbAdapter;
pub use scanner::AdbScannerGateway;

use crate::client::AdbClient;
use crate::repositories::{
    AdbAppRepository, AdbDataRepository, AdbDeviceRepository, AdbScannerRepository,
};

/// FACTORY Pattern: Centralizes the creation of ADB-related gateways.
pub struct AdbGatewayFactory {
    client: AdbClient,
}

impl AdbGatewayFactory {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    pub fn create_device_gateway(&self) -> AdbDeviceGateway {
        AdbDeviceGateway::new(AdbDeviceRepository::new(self.client.clone()))
    }

    pub fn create_app_gateway(&self) -> AdbAppGateway {
        AdbAppGateway::new(AdbAppRepository::new(self.client.clone()))
    }

    pub fn create_scanner_gateway(&self) -> AdbScannerGateway {
        AdbScannerGateway::new(AdbScannerRepository::new(self.client.clone()))
    }

    pub fn create_data_gateway(&self) -> AdbDataGateway {
        AdbDataGateway::new(AdbDataRepository::new(self.client.clone()))
    }

    /// Creates the main Facade containing all gateways.
    pub fn create_adapter(self) -> AdbAdapter {
        AdbAdapter {
            client: self.client.clone(),
            device_gw: self.create_device_gateway(),
            app_gw: self.create_app_gateway(),
            scanner_gw: self.create_scanner_gateway(),
            data_gw: self.create_data_gateway(),
        }
    }
}
