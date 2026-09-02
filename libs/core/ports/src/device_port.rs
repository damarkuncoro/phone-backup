use anyhow::Result;
use domain::{CapabilityMatrix, Device, DeviceId};

/// Anything capable of talking to a physical device is a `DevicePort`.
///
/// Concrete implementations (Phase 02+): `AdbDeviceAdapter`,
/// `MtpDeviceAdapter`, and for tests, `MockDeviceAdapter`.
///
/// `application::BackupService` depends only on this trait, never on
/// a concrete adapter — swapping ADB for MTP (or adding an iOS
/// adapter later) never touches business logic.
pub trait DevicePort: Send + Sync {
    /// Enumerate devices currently reachable through this adapter.
    fn discover(&self) -> Result<Vec<Device>>;

    /// Fetch full info for one already-discovered device.
    fn info(&self, id: &DeviceId) -> Result<Device>;

    /// Determine what this device will actually let us read.
    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix>;

    /// Read a file's content from the device.
    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>>;

    /// Push a file to the device.
    fn push_file(
        &self,
        id: &DeviceId,
        source: &mut dyn std::io::Read,
        target_path: &str,
    ) -> Result<()>;

    /// Check device battery level and temperature.
    fn battery_status(&self, id: &DeviceId) -> Result<(u32, f32)>;

    /// List entries in a specific directory.
    fn list_directory(&self, id: &DeviceId, path: &str) -> Result<Vec<domain::FileEntry>>;

    /// Delete a file or directory on the device.
    fn delete_remote(&self, id: &DeviceId, path: &str) -> Result<()>;

    /// Rename/Move a file or directory on the device.
    fn rename_remote(&self, id: &DeviceId, old_path: &str, new_path: &str) -> Result<()>;

    /// Copy a file or directory on the device.
    fn copy_remote(&self, id: &DeviceId, source_path: &str, target_path: &str) -> Result<()>;

    /// Calculate SHA-256 hash of a remote file.
    fn calculate_hash(&self, id: &DeviceId, path: &str) -> Result<String>;
}
