use anyhow::Result;
use domain::{CapabilityMatrix, Device, DeviceId, FileEntry};
use ports::{DevicePort, ScannerPort};
use std::sync::Arc;
use tracing::{info, error};

#[derive(Clone)]
pub struct CompositeDeviceAdapter {
    adb_device: Arc<dyn DevicePort>,
    mtp_device: Arc<dyn DevicePort>,
}

impl CompositeDeviceAdapter {
    pub fn new(adb_device: Arc<dyn DevicePort>, mtp_device: Arc<dyn DevicePort>) -> Self {
        Self {
            adb_device,
            mtp_device,
        }
    }

    fn select_adapter(&self, id: &DeviceId) -> &Arc<dyn DevicePort> {
        if id.0.starts_with("mtp:") || id.0.starts_with("usb://") {
            &self.mtp_device
        } else {
            &self.adb_device
        }
    }
}

impl DevicePort for CompositeDeviceAdapter {
    fn discover(&self) -> Result<Vec<Device>> {
        let mut all = Vec::new();
        let mut seen_serials = std::collections::HashSet::new();

        // RECOMMENDATION 10: ADB Priority
        match self.adb_device.discover() {
            Ok(adb_devs) => {
                info!("Composite Discovery: Found {} ADB devices", adb_devs.len());
                for dev in adb_devs {
                    seen_serials.insert(dev.serial.clone());
                    all.push(dev);
                }
            },
            Err(e) => error!("Composite Discovery: ADB discovery failed: {}", e),
        }

        match self.mtp_device.discover() {
            Ok(mtp_devs) => {
                info!("Composite Discovery: Found {} MTP devices", mtp_devs.len());
                for dev in mtp_devs {
                    // Only add if not already present via ADB
                    if !seen_serials.contains(&dev.serial) {
                        all.push(dev);
                    } else {
                        info!("Composite Discovery: Skipping MTP for device {} (already available via ADB)", dev.serial);
                    }
                }
            },
            Err(e) => error!("Composite Discovery: MTP discovery failed: {}", e),
        }
        Ok(all)
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        self.select_adapter(id).info(id)
    }

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        self.select_adapter(id).capabilities(id)
    }

    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> {
        self.select_adapter(id).read_file(id, path)
    }

    fn push_file(&self, id: &DeviceId, source: &mut dyn std::io::Read, target_path: &str) -> Result<()> {
        self.select_adapter(id).push_file(id, source, target_path)
    }

    fn battery_status(&self, id: &DeviceId) -> Result<(u32, f32)> {
        self.select_adapter(id).battery_status(id)
    }

    fn list_directory(&self, id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        self.select_adapter(id).list_directory(id, path)
    }

    fn delete_remote(&self, id: &DeviceId, path: &str) -> Result<()> {
        self.select_adapter(id).delete_remote(id, path)
    }

    fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()> {
        self.select_adapter(id).rename_remote(id, old_path, new_path)
    }

    fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        self.select_adapter(id).copy_remote(id, source_path, target_path)
    }

    fn calculate_hash(&self, id: &DeviceId, path: &str) -> Result<String> {
        self.select_adapter(id).calculate_hash(id, path)
    }
}

#[derive(Clone)]
pub struct CompositeScannerAdapter {
    adb_scanner: Arc<dyn ScannerPort>,
    mtp_scanner: Arc<dyn ScannerPort>,
}

impl CompositeScannerAdapter {
    pub fn new(adb_scanner: Arc<dyn ScannerPort>, mtp_scanner: Arc<dyn ScannerPort>) -> Self {
        Self {
            adb_scanner,
            mtp_scanner,
        }
    }
}

impl ScannerPort for CompositeScannerAdapter {
    fn scan(&self, id: &DeviceId, target_paths: Vec<String>) -> Result<Vec<FileEntry>> {
        if id.0.starts_with("mtp:") || id.0.starts_with("usb://") {
            self.mtp_scanner.scan(id, target_paths)
        } else {
            self.adb_scanner.scan(id, target_paths)
        }
    }
}
