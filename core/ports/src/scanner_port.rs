use anyhow::Result;
use domain::{DeviceId, FileEntry};

/// Port for scanning a device's filesystem.
pub trait ScannerPort: Send + Sync {
    /// Recursively scan the device's storage for files.
    ///
    /// Depending on capabilities, this may scan internal storage,
    /// SD cards, or specific media folders.
    ///
    /// If `roots` is provided, only scan those specific directory paths.
    fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>>;
}
