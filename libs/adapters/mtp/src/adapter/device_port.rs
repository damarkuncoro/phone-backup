use anyhow::{anyhow, Result};
use domain::{Capability, CapabilityMatrix, CapabilityStatus, ConnectionType, Device, DeviceId, FileEntry};
use ports::DevicePort;
use tracing::instrument;

use super::MtpAdapter;

impl DevicePort for MtpAdapter {
    #[instrument(skip(self))]
    fn discover(&self) -> Result<Vec<Device>> {
        let mounts = self.get_active_mounts();
        let mut devices = Vec::new();

        for (idx, mount) in mounts.into_iter().enumerate() {
            let total_space = 64 * 1024 * 1024 * 1024;
            let free_space = 20 * 1024 * 1024 * 1024;

            let id = if mount.path.to_string_lossy().starts_with("usb://") {
                DeviceId::new(mount.path.to_string_lossy().into_owned())
            } else {
                DeviceId::new(format!("mtp:device_{}", idx + 1))
            };

            devices.push(Device {
                id,
                manufacturer: "Android (MTP)".to_string(),
                model: mount.name,
                serial: mount.path.to_string_lossy().into_owned(),
                os_version: "Media Transfer Protocol".to_string(),
                sdk_version: None,
                storage_total_bytes: total_space,
                storage_used_bytes: total_space.saturating_sub(free_space),
                storage_free_bytes: free_space,
                connection_type: ConnectionType::Mtp,
            });
        }
        Ok(devices)
    }

    #[instrument(skip(self))]
    fn info(&self, id: &DeviceId) -> Result<Device> {
        let mut device = {
            let devices = self.discover()?;
            devices
                .into_iter()
                .find(|d| &d.id == id)
                .ok_or_else(|| anyhow!("MTP Device {} not found", id))?
        };

        if id.0.starts_with("usb://") {
            if let Ok(ops) = self.get_native_ops(id) {
                if let Ok((total, free)) = ops.get_storage_info() {
                    device.storage_total_bytes = total;
                    device.storage_free_bytes = free;
                    device.storage_used_bytes = total.saturating_sub(free);
                }
            }
        }

        Ok(device)
    }

    #[instrument(skip(self))]
    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        let mut matrix = CapabilityMatrix::new();

        if id.0.starts_with("usb://") {
            if let Ok(_ops) = self.get_native_ops(id) {
                matrix.set(Capability::ReadFiles, CapabilityStatus::Available);
                matrix.set(Capability::ReadMedia, CapabilityStatus::Available);
                matrix.set(Capability::ReadDownload, CapabilityStatus::Available);
                matrix.set(Capability::ReadDocuments, CapabilityStatus::Available);
            }
        } else {
            matrix.set(Capability::ReadFiles, CapabilityStatus::Available);
            matrix.set(Capability::ReadMedia, CapabilityStatus::Available);
        }

        Ok(matrix)
    }

    #[instrument(skip(self))]
    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> {
        if id.0.starts_with("usb://") {
            self.get_native_ops(id)?.read_file(path)
        } else {
            self.get_fs_ops(id)?.read_file(path)
        }
    }

    #[instrument(skip(self, source))]
    fn push_file(&self, id: &DeviceId, source: &mut dyn std::io::Read, target_path: &str) -> Result<()> {
        if id.0.starts_with("usb://") {
            self.get_native_ops(id)?.push_file(source, target_path)
        } else {
            self.get_fs_ops(id)?.push_file(source, target_path)
        }
    }

    #[instrument(skip(self))]
    fn battery_status(&self, _id: &DeviceId) -> Result<(u32, f32)> {
        Ok((100, 28.0))
    }

    #[instrument(skip(self))]
    fn list_directory(&self, id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        if id.0.starts_with("usb://") {
            self.get_native_ops(id)?.list_directory(id, path)
        } else {
            self.get_fs_ops(id)?.list_directory(id, path)
        }
    }

    #[instrument(skip(self))]
    fn delete_remote(&self, id: &DeviceId, path: &str) -> Result<()> {
        if id.0.starts_with("usb://") {
            self.get_native_ops(id)?.delete_object(path)
        } else {
            self.get_fs_ops(id)?.delete(path)
        }
    }

    #[instrument(skip(self))]
    fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()> {
        if id.0.starts_with("usb://") {
            self.get_native_ops(id)?.rename_object(old_path, new_path)
        } else {
            self.get_fs_ops(id)?.rename(old_path, new_path)
        }
    }

    #[instrument(skip(self))]
    fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        if id.0.starts_with("usb://") {
            let mut reader = self.read_file(id, source_path)?;
            self.push_file(id, &mut reader, target_path)
        } else {
            self.get_fs_ops(id)?.copy(source_path, target_path)
        }
    }

    #[instrument(skip(self))]
    fn calculate_hash(&self, id: &DeviceId, path: &str) -> Result<String> {
        if id.0.starts_with("usb://") {
            self.get_native_ops(id)?.calculate_quick_hash(path)
        } else {
            use sha2::{Digest, Sha256};
            let mut reader = self.read_file(id, path)?;
            let mut hasher = Sha256::new();
            std::io::copy(&mut reader, &mut hasher)?;
            Ok(format!("{:x}", hasher.finalize()))
        }
    }
}
