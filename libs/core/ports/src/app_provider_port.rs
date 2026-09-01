use anyhow::Result;
use domain::{AppInfo, DeviceId};

pub trait AppProviderPort: Send + Sync {
    /// List all installed applications on the device.
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>>;

    /// Extract the APK for a specific app.
    fn get_apk(&self, device_id: &DeviceId, package_name: &str) -> Result<Box<dyn std::io::Read>>;

    /// Install an APK to the device.
    fn install_app(&self, device_id: &DeviceId, apk_data: &mut dyn std::io::Read) -> Result<()>;
}
