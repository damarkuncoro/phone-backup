use anyhow::Result;
use domain::{AppId, AppInfo, DeviceId};
use ports::AppProviderPort;

pub struct MockAppProvider;

impl AppProviderPort for MockAppProvider {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> {
        Ok(vec![
            AppInfo {
                id: AppId("com.whatsapp".into()),
                device_id: device_id.clone(),
                package_name: "com.whatsapp".into(),
                version_name: "2.24.1".into(),
                version_code: 2401,
                installer: Some("com.android.vending".into()),
                app_name: "WhatsApp".into(),
            },
            AppInfo {
                id: AppId("com.instagram.android".into()),
                device_id: device_id.clone(),
                package_name: "com.instagram.android".into(),
                version_name: "315.0.0".into(),
                version_code: 31500,
                installer: Some("com.android.vending".into()),
                app_name: "Instagram".into(),
            },
        ])
    }

    fn get_apk(
        &self,
        _device_id: &DeviceId,
        _package_name: &str,
    ) -> Result<Box<dyn std::io::Read>> {
        let content = vec![0u8; 1024];
        Ok(Box::new(std::io::Cursor::new(content)))
    }

    fn install_app(&self, _device_id: &DeviceId, _apk_data: &mut dyn std::io::Read) -> Result<()> {
        Ok(())
    }
}
