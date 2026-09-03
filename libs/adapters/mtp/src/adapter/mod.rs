pub mod device_port;
pub mod scanner_port;

use anyhow::Result;
use domain::DeviceId;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::discovery::{DiscoveryOrchestrator, MtpMount};
use crate::native::NativeMtpOperations;
use crate::operations::MtpFileOperations;

#[derive(Clone)]
pub struct MtpAdapter {
    custom_root: Option<PathBuf>,
    discovery: Arc<DiscoveryOrchestrator>,
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

    pub(crate) fn get_active_mounts(&self) -> Vec<MtpMount> {
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

    pub(crate) fn get_native_ops(&self, id: &DeviceId) -> Result<NativeMtpOperations> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(ops) = sessions.get(&id.0) {
            return Ok(ops.clone());
        }

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

    pub(crate) fn get_fs_ops(&self, _id: &DeviceId) -> Result<MtpFileOperations> {
        let mounts = self.get_active_mounts();
        let fs_mounts: Vec<_> = mounts
            .iter()
            .filter(|m| !m.path.to_string_lossy().starts_with("usb://"))
            .collect();
        let path = fs_mounts
            .first()
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
