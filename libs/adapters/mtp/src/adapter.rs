use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use domain::{Capability, CapabilityMatrix, CapabilityStatus, ConnectionType, Device, DeviceId, FileEntry};
use ports::{DevicePort, ScannerPort};
use tracing::{warn, instrument};

use crate::discovery::{DiscoveryOrchestrator, MtpMount};
use crate::operations::MtpFileOperations;
use crate::native_ops::NativeMtpOperations;
use crate::scanner::MtpScanner;

#[derive(Clone)]
pub struct MtpAdapter {
    custom_root: Option<PathBuf>,
    discovery: Arc<DiscoveryOrchestrator>,
    // Session cache to prevent "device busy" errors from multiple opens
    sessions: Arc<Mutex<HashMap<String, NativeMtpOperations>>>,
}

impl MtpAdapter {
    pub fn new() -> Self {
        Self {
            custom_root: None,
            discovery: Arc::new(DiscoveryOrchestrator::new()),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_root(root: impl Into<PathBuf>) -> Self {
        Self {
            custom_root: Some(root.into()),
            discovery: Arc::new(DiscoveryOrchestrator::new()),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_active_mounts(&self) -> Vec<MtpMount> {
        if let Some(ref root) = self.custom_root {
            if root.exists() {
                return vec![MtpMount {
                    name: "MTP Virtual Storage".to_string(),
                    path: root.clone(),
                }];
            }
        }
        self.discovery.discover()
    }

    fn get_native_ops(&self, id: &DeviceId) -> Result<NativeMtpOperations> {
        let mut sessions = self.sessions.lock().unwrap();

        // Return cached session if available
        if let Some(ops) = sessions.get(&id.0) {
            return Ok(ops.clone());
        }

        // Create new session
        let ops = if id.0.contains("serial/") {
            let serial = id.0.split("serial/").last().unwrap_or("");
            NativeMtpOperations::new_from_serial(serial.to_string())?
        } else if id.0.contains("location/") {
            let loc_str = id.0.split("location/").last().unwrap_or("0");
            let loc = loc_str.parse::<u64>().unwrap_or(0);
            NativeMtpOperations::new_from_location(loc)?
        } else {
            anyhow::bail!("Invalid native MTP ID format")
        };

        sessions.insert(id.0.clone(), ops.clone());
        Ok(ops)
    }

    fn get_fs_ops(&self, _id: &DeviceId) -> Result<MtpFileOperations> {
        let mounts = self.get_active_mounts();
        let fs_mounts: Vec<_> = mounts.iter().filter(|m| !m.path.to_string_lossy().starts_with("usb://")).collect();
        let path = fs_mounts.first()
            .map(|m| m.path.clone())
            .unwrap_or_else(|| PathBuf::from("/sdcard"));
        Ok(MtpFileOperations::new(path))
    }
}

impl Default for MtpAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl DevicePort for MtpAdapter {
    #[instrument(skip(self))]
    fn discover(&self) -> Result<Vec<Device>> {
        let mounts = self.get_active_mounts();
        let mut devices = Vec::new();

        for (idx, mount) in mounts.into_iter().enumerate() {
            let total_space = 64 * 1024 * 1024 * 1024; // Placeholder
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
        let devices = self.discover()?;
        devices.into_iter()
            .find(|d| &d.id == id)
            .ok_or_else(|| anyhow!("MTP Device {} not found", id))
    }

    #[instrument(skip(self))]
    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        let mut matrix = CapabilityMatrix::new();

        if id.0.starts_with("usb://") {
            // RECOMMENDATION 11: Dynamic Capability Detection
            if let Ok(_ops) = self.get_native_ops(id) {
                // In a real implementation, we'd query the MtpDevice handle
                // For now, set baseline for native USB
                matrix.set(Capability::ReadFiles, CapabilityStatus::Available);
                matrix.set(Capability::ReadMedia, CapabilityStatus::Available);
                matrix.set(Capability::ReadDownload, CapabilityStatus::Available);
                matrix.set(Capability::ReadDocuments, CapabilityStatus::Available);
            }
        } else {
            // Filesystem based MTP
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
            anyhow::bail!("Push to native MTP not yet implemented")
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
            anyhow::bail!("Delete on native MTP not yet implemented")
        } else {
            self.get_fs_ops(id)?.delete(path)
        }
    }

    #[instrument(skip(self))]
    fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()> {
        if id.0.starts_with("usb://") {
            anyhow::bail!("Rename on native MTP not yet implemented")
        } else {
            self.get_fs_ops(id)?.rename(old_path, new_path)
        }
    }

    #[instrument(skip(self))]
    fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        if id.0.starts_with("usb://") {
            anyhow::bail!("Copy on native MTP not yet implemented")
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

impl ScannerPort for MtpAdapter {
    #[instrument(skip(self))]
    fn scan(&self, id: &DeviceId, target_paths: Vec<String>) -> Result<Vec<FileEntry>> {
        if id.0.starts_with("usb://") {
            let ops = self.get_native_ops(id)?;
            ops.scan_recursive(id, target_paths)
        } else {
            let paths_to_scan = if target_paths.is_empty() {
                vec!["/".to_string()]
            } else {
                target_paths
            };
            let mounts = self.get_active_mounts();
            let fs_mounts: Vec<_> = mounts.iter().filter(|m| !m.path.to_string_lossy().starts_with("usb://")).collect();
            let path = fs_mounts.first()
                .map(|m| m.path.clone())
                .unwrap_or_else(|| PathBuf::from("/sdcard"));

            MtpScanner::new(path).scan(id, paths_to_scan)
        }
    }
}
