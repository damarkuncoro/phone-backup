use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Arc;
use domain::{Capability, CapabilityMatrix, CapabilityStatus, ConnectionType, Device, DeviceId, FileEntry};
use ports::{DevicePort, ScannerPort};
use tracing::instrument;

use crate::discovery::{DiscoveryOrchestrator, MtpMount};
use crate::operations::MtpFileOperations;
use crate::scanner::MtpScanner;

#[derive(Clone)]
pub struct MtpAdapter {
    custom_root: Option<PathBuf>,
    discovery: Arc<DiscoveryOrchestrator>,
}

impl MtpAdapter {
    pub fn new() -> Self {
        Self {
            custom_root: None,
            discovery: Arc::new(DiscoveryOrchestrator::new()),
        }
    }

    pub fn with_root(root: impl Into<PathBuf>) -> Self {
        Self {
            custom_root: Some(root.into()),
            discovery: Arc::new(DiscoveryOrchestrator::new()),
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

    fn get_ops(&self, _id: &DeviceId) -> Result<MtpFileOperations> {
        let mounts = self.get_active_mounts();
        let path = mounts.first()
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

            devices.push(Device {
                id: DeviceId::new(format!("mtp:device_{}", idx + 1)),
                manufacturer: "Android (MTP)".to_string(),
                model: mount.name,
                serial: format!("MTP-{:04}", idx + 1),
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
    fn capabilities(&self, _id: &DeviceId) -> Result<CapabilityMatrix> {
        let mut matrix = CapabilityMatrix::new();
        matrix.set(Capability::ReadFiles, CapabilityStatus::Available);
        matrix.set(Capability::ReadMedia, CapabilityStatus::Available);
        matrix.set(Capability::ReadDownload, CapabilityStatus::Available);
        matrix.set(Capability::ReadDocuments, CapabilityStatus::Available);
        Ok(matrix)
    }

    #[instrument(skip(self))]
    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> {
        self.get_ops(id)?.read_file(path)
    }

    #[instrument(skip(self, source))]
    fn push_file(&self, id: &DeviceId, source: &mut dyn std::io::Read, target_path: &str) -> Result<()> {
        self.get_ops(id)?.push_file(source, target_path)
    }

    #[instrument(skip(self))]
    fn battery_status(&self, _id: &DeviceId) -> Result<(u32, f32)> {
        Ok((100, 28.0))
    }

    #[instrument(skip(self))]
    fn list_directory(&self, id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        self.get_ops(id)?.list_directory(id, path)
    }

    #[instrument(skip(self))]
    fn delete_remote(&self, id: &DeviceId, path: &str) -> Result<()> {
        self.get_ops(id)?.delete(path)
    }

    #[instrument(skip(self))]
    fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()> {
        self.get_ops(id)?.rename(old_path, new_path)
    }

    #[instrument(skip(self))]
    fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()> {
        self.get_ops(id)?.copy(source_path, target_path)
    }

    #[instrument(skip(self))]
    fn calculate_hash(&self, id: &DeviceId, path: &str) -> Result<String> {
        use sha2::{Digest, Sha256};
        let mut reader = self.read_file(id, path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut reader, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }
}

impl ScannerPort for MtpAdapter {
    #[instrument(skip(self))]
    fn scan(&self, id: &DeviceId, target_paths: Vec<String>) -> Result<Vec<FileEntry>> {
        let mounts = self.get_active_mounts();
        let path = mounts.first()
            .map(|m| m.path.clone())
            .unwrap_or_else(|| PathBuf::from("/sdcard"));

        MtpScanner::new(path).scan(id, target_paths)
    }
}
