use crate::repositories::AdbDeviceRepository;
use anyhow::Result;
use domain::{CapabilityMatrix, Device, DeviceId};
use ports::DevicePort;

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

    fn push_file(
        &self,
        id: &DeviceId,
        source: &mut dyn std::io::Read,
        target_path: &str,
    ) -> Result<()> {
        self.repo.push_file(id, source, target_path)
    }

    fn battery_status(&self, id: &DeviceId) -> Result<(u32, f32)> {
        self.repo.get_battery_status(id)
    }

    fn list_directory(&self, id: &DeviceId, path: &str) -> Result<Vec<domain::FileEntry>> {
        self.repo.list_directory(id, path)
    }

    fn delete_remote(&self, id: &DeviceId, path: &str) -> Result<()> {
        self.repo.delete_remote(id, path)
    }

    fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()> {
        self.repo.rename_remote(id, old_path, new_path)
    }

    fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        self.repo.copy_remote(id, source_path, target_path)
    }

    fn calculate_hash(&self, id: &DeviceId, path: &str) -> Result<String> {
        self.repo.calculate_hash(id, path)
    }

    fn set_stay_on(&self, id: &DeviceId, stay_on: bool) -> Result<()> {
        self.repo.set_stay_on(id, stay_on)
    }
}
