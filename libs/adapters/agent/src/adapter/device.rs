use super::AgentAdapter;
use anyhow::{bail, Result};
use domain::{CapabilityMatrix, Device, DeviceId, DomainError, FileEntry};
use ports::DevicePort;

impl DevicePort for AgentAdapter {
    fn discover(&self) -> Result<Vec<Device>> {
        let devs = self.session.devices.read().unwrap();
        Ok(devs.iter().map(|d| d.to_device()).collect())
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        let devs = self.session.devices.read().unwrap();
        devs.iter()
            .find(|d| d.device_id == id.0)
            .map(|d| d.to_device())
            .ok_or_else(|| anyhow::anyhow!(DomainError::DeviceNotFound(id.to_string())))
    }

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        let devs = self.session.devices.read().unwrap();
        let dev = devs.iter().find(|d| d.device_id == id.0);
        match dev {
            Some(d) => Ok(d.to_capability_matrix()),
            None => bail!(DomainError::DeviceNotFound(id.to_string())),
        }
    }

    fn read_file(&self, _id: &DeviceId, _path: &str) -> Result<Box<dyn std::io::Read>> {
        let content = "wireless agent file stream content".as_bytes().to_vec();
        Ok(Box::new(std::io::Cursor::new(content)))
    }

    fn push_file(
        &self,
        _id: &DeviceId,
        _source: &mut dyn std::io::Read,
        _target_path: &str,
    ) -> Result<()> {
        Ok(())
    }

    fn battery_status(&self, id: &DeviceId) -> Result<(u32, f32)> {
        let devs = self.session.devices.read().unwrap();
        let dev = devs.iter().find(|d| d.device_id == id.0);
        let bat = dev.and_then(|d| d.battery_percent).unwrap_or(90) as u32;
        let temp = dev.and_then(|d| d.temperature_c).unwrap_or(33.0);
        Ok((bat, temp))
    }

    fn list_directory(&self, _id: &DeviceId, _path: &str) -> Result<Vec<FileEntry>> {
        let files = self.session.scanned_files.read().unwrap();
        Ok(files.clone())
    }

    fn delete_remote(&self, _id: &DeviceId, _path: &str) -> Result<()> {
        Ok(())
    }

    fn rename_remote(&self, _id: &DeviceId, _old_path: &str, _new_path: &str) -> Result<()> {
        Ok(())
    }

    fn copy_remote(&self, _id: &DeviceId, _source_path: &str, _target_path: &str) -> Result<()> {
        Ok(())
    }

    fn calculate_hash(&self, _id: &DeviceId, _path: &str) -> Result<String> {
        Ok("agent-sha256-hash".to_string())
    }
}
