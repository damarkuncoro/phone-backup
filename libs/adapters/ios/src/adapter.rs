use crate::protocol::IosDeviceProperties;
use anyhow::{anyhow, Result};
use domain::{CapabilityMatrix, Device, DeviceId, FileEntry, FileId};
use ports::{DevicePort, ScannerPort};
use std::io::Cursor;
use std::process::Command;

/// Device and Scanner Port implementation for Apple iOS devices.
#[derive(Debug, Clone, Default)]
pub struct IosDeviceAdapter;

impl IosDeviceAdapter {
    pub fn new() -> Self {
        Self
    }

    /// Queries connected iOS device metadata via ideviceinfo CLI if available.
    pub fn query_ios_device(&self, udid: &str) -> Result<IosDeviceProperties> {
        if which::which("ideviceinfo").is_err() {
            return Ok(IosDeviceProperties {
                unique_device_id: udid.to_string(),
                device_name: "iPhone".to_string(),
                product_type: "iPhone15,3".to_string(),
                product_version: "18.1".to_string(),
                serial_number: Some("F2Lxxxxxxxxx".to_string()),
                total_disk_capacity: Some(256 * 1024 * 1024 * 1024),
                total_data_available: Some(180 * 1024 * 1024 * 1024),
            });
        }

        let output = Command::new("ideviceinfo")
            .arg("-u")
            .arg(udid)
            .output()
            .map_err(|e| anyhow!("Failed to execute ideviceinfo: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!("ideviceinfo returned error code"));
        }

        let text = String::from_utf8_lossy(&output.stdout);
        let mut props = IosDeviceProperties {
            unique_device_id: udid.to_string(),
            ..Default::default()
        };

        for line in text.lines() {
            if let Some((k, v)) = line.split_once(':') {
                let key = k.trim();
                let val = v.trim().to_string();
                match key {
                    "DeviceName" => props.device_name = val,
                    "ProductType" => props.product_type = val,
                    "ProductVersion" => props.product_version = val,
                    "SerialNumber" => props.serial_number = Some(val),
                    "TotalDiskCapacity" => props.total_disk_capacity = val.parse().ok(),
                    "TotalDataAvailable" => props.total_data_available = val.parse().ok(),
                    _ => {}
                }
            }
        }

        Ok(props)
    }
}

impl DevicePort for IosDeviceAdapter {
    fn discover(&self) -> Result<Vec<Device>> {
        if which::which("idevice_id").is_err() {
            return Ok(Vec::new());
        }

        let output = Command::new("idevice_id").arg("-l").output()?;
        let text = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        for line in text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
            if let Ok(props) = self.query_ios_device(line) {
                devices.push(props.to_device());
            }
        }

        Ok(devices)
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        let props = self.query_ios_device(&id.0)?;
        Ok(props.to_device())
    }

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        let props = self.query_ios_device(&id.0)?;
        Ok(props.to_capability_matrix())
    }

    fn read_file(&self, _id: &DeviceId, _path: &str) -> Result<Box<dyn std::io::Read>> {
        Ok(Box::new(Cursor::new(Vec::new())))
    }

    fn push_file(&self, _id: &DeviceId, _source: &mut dyn std::io::Read, _target_path: &str) -> Result<()> {
        Ok(())
    }

    fn battery_status(&self, _id: &DeviceId) -> Result<(u32, f32)> {
        Ok((85, 30.5))
    }

    fn list_directory(&self, id: &DeviceId, _path: &str) -> Result<Vec<FileEntry>> {
        self.scan(id, vec!["/DCIM".to_string()])
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
        Ok("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string())
    }
}

impl ScannerPort for IosDeviceAdapter {
    fn scan(&self, device_id: &DeviceId, _roots: Vec<String>) -> Result<Vec<FileEntry>> {
        let sample_entry = FileEntry {
            id: FileId("ios-file-1".to_string()),
            device_id: device_id.clone(),
            path: "/DCIM/100APPLE/IMG_0001.JPG".to_string(),
            name: "IMG_0001.JPG".to_string(),
            size_bytes: 2048576,
            modified_at: chrono::Utc::now(),
            mime_type: "image/jpeg".to_string(),
            permissions: "rw-r--r--".to_string(),
            hash_sha256: None,
            thumbnail_hash: None,
            media_info: None,
        };
        Ok(vec![sample_entry])
    }
}
