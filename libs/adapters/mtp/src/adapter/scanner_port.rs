use anyhow::Result;
use domain::{DeviceId, FileEntry};
use ports::ScannerPort;
use std::path::PathBuf;
use tracing::instrument;

use super::MtpAdapter;
use crate::scanner::MtpScanner;

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
            let fs_mounts: Vec<_> = mounts
                .iter()
                .filter(|m| !m.path.to_string_lossy().starts_with("usb://"))
                .collect();
            let path = fs_mounts
                .first()
                .map(|m| m.path.clone())
                .unwrap_or_else(|| PathBuf::from("/sdcard"));

            MtpScanner::new(path).scan(id, paths_to_scan)
        }
    }
}
