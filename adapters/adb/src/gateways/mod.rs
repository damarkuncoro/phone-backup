pub mod device;
pub mod app;
pub mod scanner;
pub mod data;

pub use device::AdbDeviceGateway;
pub use app::AdbAppGateway;
pub use scanner::AdbScannerGateway;
pub use data::AdbDataGateway;

use crate::client::{AdbClient, AdbMonitor};
use anyhow::Result;
use domain::{Device, DeviceId, CapabilityMatrix, FileEntry, AppInfo, Contact, Sms, CallLog};
use ports::{DevicePort, ScannerPort, AppProviderPort, DataProviderPort};

use crate::repositories::{
    AdbDeviceRepository, AdbAppRepository, AdbScannerRepository, AdbDataRepository
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

/// FACADE Pattern: A unified interface that aggregates all ADB sub-systems.
/// It implements all relevant Ports by delegating to specialized gateways.
#[derive(Clone)]
pub struct AdbAdapter {
    pub client: AdbClient,
    device_gw: AdbDeviceGateway,
    app_gw: AdbAppGateway,
    scanner_gw: AdbScannerGateway,
    data_gw: AdbDataGateway,
}

impl AdbAdapter {
    pub fn new(client: AdbClient) -> Self {
        AdbGatewayFactory::new(client).create_adapter()
    }

    pub fn monitor(&self) -> AdbMonitor {
        self.client.monitor()
    }
}

impl DevicePort for AdbAdapter {
    fn discover(&self) -> Result<Vec<Device>> { self.device_gw.discover() }
    fn info(&self, id: &DeviceId) -> Result<Device> { self.device_gw.info(id) }
    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> { self.device_gw.capabilities(id) }
    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> { self.device_gw.read_file(id, path) }
    fn push_file(&self, id: &DeviceId, source: &mut dyn std::io::Read, target_path: &str) -> Result<()> { self.device_gw.push_file(id, source, target_path) }
    fn battery_status(&self, id: &DeviceId) -> Result<(u32, f32)> { self.device_gw.battery_status(id) }
}

impl ScannerPort for AdbAdapter {
    fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> { self.scanner_gw.scan(device_id, roots) }
}

impl AppProviderPort for AdbAdapter {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> { self.app_gw.list_apps(device_id) }
    fn get_apk(&self, device_id: &DeviceId, package_name: &str) -> Result<Box<dyn std::io::Read>> { self.app_gw.get_apk(device_id, package_name) }
    fn install_app(&self, device_id: &DeviceId, apk_data: &mut dyn std::io::Read) -> Result<()> { self.app_gw.install_app(device_id, apk_data) }
}

impl DataProviderPort for AdbAdapter {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>> { self.data_gw.list_contacts(device_id) }
    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>> { self.data_gw.list_sms(device_id) }
    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>> { self.data_gw.list_call_logs(device_id) }
}
