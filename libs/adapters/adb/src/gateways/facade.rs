use crate::client::{AdbClient, AdbMonitor};
use crate::gateways::{
    AdbAppGateway, AdbDataGateway, AdbDeviceGateway, AdbGatewayFactory, AdbScannerGateway,
};
use anyhow::Result;
use domain::{AppInfo, CallLog, CapabilityMatrix, Contact, Device, DeviceId, FileEntry, Sms};
use ports::{AppProviderPort, DataProviderPort, DevicePort, ScannerPort};

/// FACADE Pattern: A unified interface that aggregates all ADB sub-systems.
/// It implements all relevant Ports by delegating to specialized gateways.
#[derive(Clone)]
pub struct AdbAdapter {
    pub client: AdbClient,
    pub(crate) device_gw: AdbDeviceGateway,
    pub(crate) app_gw: AdbAppGateway,
    pub(crate) scanner_gw: AdbScannerGateway,
    pub(crate) data_gw: AdbDataGateway,
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
    fn discover(&self) -> Result<Vec<Device>> {
        self.device_gw.discover()
    }
    fn info(&self, id: &DeviceId) -> Result<Device> {
        self.device_gw.info(id)
    }
    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        self.device_gw.capabilities(id)
    }
    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> {
        self.device_gw.read_file(id, path)
    }
    fn push_file(
        &self,
        id: &DeviceId,
        source: &mut dyn std::io::Read,
        target_path: &str,
    ) -> Result<()> {
        self.device_gw.push_file(id, source, target_path)
    }
    fn battery_status(&self, id: &DeviceId) -> Result<(u32, f32)> {
        self.device_gw.battery_status(id)
    }

    fn list_directory(&self, id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        self.device_gw.list_directory(id, path)
    }
    fn delete_remote(&self, id: &DeviceId, path: &str) -> Result<()> {
        self.device_gw.delete_remote(id, path)
    }
    fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()> {
        self.device_gw.rename_remote(id, old_path, new_path)
    }
    fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        self.device_gw.copy_remote(id, source_path, target_path)
    }
    fn calculate_hash(&self, id: &DeviceId, path: &str) -> Result<String> {
        self.device_gw.calculate_hash(id, path)
    }
}

impl ScannerPort for AdbAdapter {
    fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        self.scanner_gw.scan(device_id, roots)
    }
}

impl AppProviderPort for AdbAdapter {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> {
        self.app_gw.list_apps(device_id)
    }
    fn get_apk(&self, device_id: &DeviceId, package_name: &str) -> Result<Box<dyn std::io::Read>> {
        self.app_gw.get_apk(device_id, package_name)
    }
    fn install_app(&self, device_id: &DeviceId, apk_data: &mut dyn std::io::Read) -> Result<()> {
        self.app_gw.install_app(device_id, apk_data)
    }
}

impl DataProviderPort for AdbAdapter {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>> {
        self.data_gw.list_contacts(device_id)
    }
    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>> {
        self.data_gw.list_sms(device_id)
    }
    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>> {
        self.data_gw.list_call_logs(device_id)
    }
}
