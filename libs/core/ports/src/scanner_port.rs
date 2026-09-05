use anyhow::Result;
use domain::{DeviceId, FileEntry, ScanFilter, ScanResult};

/// Port for scanning a device's filesystem and media storage.
pub trait ScannerPort: Send + Sync {
    /// Recursively scan the device's storage for files.
    ///
    /// Depending on capabilities, this may scan internal storage,
    /// SD cards, or specific media folders.
    ///
    /// If `roots` is provided, only scan those specific directory paths.
    fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>>;

    /// Recursively scan the device with advanced options, categories, filters, and metrics.
    fn scan_detailed(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
        _filter: Option<&ScanFilter>,
    ) -> Result<ScanResult> {
        let files = self.scan(device_id, roots)?;
        Ok(ScanResult::new(files, Vec::new()))
    }
}
