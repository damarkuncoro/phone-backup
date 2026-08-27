use anyhow::Result;
use domain::{DeviceId, FileEntry};

/// Port for scanning a device's filesystem.
pub trait ScannerPort {
    /// Recursively scan the device's storage for files.
    ///
    /// Depending on capabilities, this may scan internal storage,
    /// SD cards, or specific media folders.
    fn scan(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>>;
}
