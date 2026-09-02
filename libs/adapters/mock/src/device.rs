use anyhow::{bail, Result};
use domain::{
    Capability, CapabilityMatrix, CapabilityStatus, ConnectionType, Device, DeviceId, DomainError,
};
use ports::DevicePort;

pub struct MockDeviceAdapter {
    devices: Vec<Device>,
}

impl Default for MockDeviceAdapter {
    fn default() -> Self {
        Self::with_device_id("A1B2C3D4")
    }
}

impl MockDeviceAdapter {
    pub fn with_device_id(id: impl Into<String>) -> Self {
        Self {
            devices: vec![Device {
                id: DeviceId::new(id),
                manufacturer: "Google".into(),
                model: "Pixel 8".into(),
                serial: "A1B2C3D4".into(),
                os_version: "Android 15".into(),
                sdk_version: Some(35),
                storage_total_bytes: 256_000_000_000,
                storage_used_bytes: 184_000_000_000,
                storage_free_bytes: 72_000_000_000,
                connection_type: ConnectionType::Usb,
            }],
        }
    }
}

impl DevicePort for MockDeviceAdapter {
    fn discover(&self) -> Result<Vec<Device>> {
        Ok(self.devices.clone())
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        self.devices
            .iter()
            .find(|d| &d.id == id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!(DomainError::DeviceNotFound(id.to_string())))
    }

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        if self.devices.iter().all(|d| &d.id != id) {
            bail!(DomainError::DeviceNotFound(id.to_string()));
        }
        let mut matrix = CapabilityMatrix::new();
        matrix.set(Capability::ReadFiles, CapabilityStatus::Available);
        matrix.set(Capability::ReadMedia, CapabilityStatus::Available);
        matrix.set(Capability::ReadDownload, CapabilityStatus::Available);
        matrix.set(Capability::ReadDocuments, CapabilityStatus::Available);
        matrix.set(
            Capability::ReadAppData,
            CapabilityStatus::RequiresUserAction,
        );
        matrix.set(
            Capability::ReadContacts,
            CapabilityStatus::RequiresUserAction,
        );
        matrix.set(Capability::ReadSms, CapabilityStatus::Denied);
        matrix.set(Capability::ReadCallLog, CapabilityStatus::Denied);
        Ok(matrix)
    }

    fn read_file(&self, _id: &DeviceId, _path: &str) -> Result<Box<dyn std::io::Read>> {
        let content = "this is mock file content".as_bytes().to_vec();
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

    fn battery_status(&self, _id: &DeviceId) -> Result<(u32, f32)> {
        Ok((85, 32.5))
    }

    fn list_directory(&self, _id: &DeviceId, _path: &str) -> Result<Vec<domain::FileEntry>> {
        Ok(vec![])
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
        Ok("mock-sha256-hash".to_string())
    }
}
