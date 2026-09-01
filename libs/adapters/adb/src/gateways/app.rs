use anyhow::Result;
use domain::{DeviceId, AppInfo};
use ports::AppProviderPort;
use crate::repositories::AdbAppRepository;

#[derive(Clone)]
pub struct AdbAppGateway {
    repo: AdbAppRepository,
}

impl AdbAppGateway {
    pub fn new(repo: AdbAppRepository) -> Self {
        Self { repo }
    }
}

impl AppProviderPort for AdbAppGateway {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> {
        self.repo.list_apps(device_id)
    }

    fn get_apk(&self, device_id: &DeviceId, package_name: &str) -> Result<Box<dyn std::io::Read>> {
        self.repo.get_apk(device_id, package_name)
    }

    fn install_app(&self, device_id: &DeviceId, apk_data: &mut dyn std::io::Read) -> Result<()> {
        self.repo.install_app(device_id, apk_data)
    }
}
