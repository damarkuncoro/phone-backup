use anyhow::Result;
use domain::{CapabilityMatrix, Device, DeviceId};
use ports::DevicePort;
use crate::repositories::AdbDeviceRepository;

#[derive(Clone)]
pub struct AdbDeviceGateway {
    repo: AdbDeviceRepository,
}

impl AdbDeviceGateway {
    pub fn new(repo: AdbDeviceRepository) -> Self {
        Self { repo }
    }
}

impl DevicePort for AdbDeviceGateway {
    fn discover(&self) -> Result<Vec<Device>> {
        self.repo.discover()
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        self.repo.get_info(id)
    }

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        self.repo.get_capabilities(id)
    }

    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> {
        self.repo.read_file(id, path)
    }

    fn push_file(&self, id: &DeviceId, source: &mut dyn std::io::Read, target_path: &str) -> Result<()> {
        self.repo.push_file(id, source, target_path)
    }

    fn battery_status(&self, id: &DeviceId) -> Result<(u32, f32)> {
        self.repo.get_battery_status(id)
    }
}
