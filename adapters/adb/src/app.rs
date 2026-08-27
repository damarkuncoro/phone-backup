use crate::client::AdbClient;
use anyhow::Result;
use domain::{AppId, AppInfo, DeviceId};
use ports::AppProviderPort;

pub struct AdbAppProvider {
    client: AdbClient,
}

impl AdbAppProvider {
    pub fn new() -> Self {
        Self {
            client: AdbClient::new(),
        }
    }
}

impl Default for AdbAppProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AppProviderPort for AdbAppProvider {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> {
        let stdout = self.client.shell(&device_id.0, "pm list packages -f --user 0")?;
        let mut apps = Vec::new();

        for line in stdout.lines() {
            if let Some(stripped) = line.strip_prefix("package:") {
                if let Some((_path, pkg)) = stripped.rsplit_once('=') {
                    apps.push(AppInfo {
                        id: AppId(pkg.to_string()),
                        device_id: device_id.clone(),
                        package_name: pkg.to_string(),
                        version_name: "Unknown".into(),
                        version_code: 0,
                        installer: None,
                        app_name: pkg.to_string(),
                    });
                }
            }
        }
        Ok(apps)
    }

    fn get_apk(&self, device_id: &DeviceId, package_name: &str) -> Result<Box<dyn std::io::Read>> {
        let stdout = self.client.shell(&device_id.0, &format!("pm path {}", package_name))?;
        if let Some(path) = stdout.lines().next().and_then(|l| l.strip_prefix("package:")) {
            let content = self.client.pull_file(&device_id.0, path)?;
            Ok(Box::new(std::io::Cursor::new(content)))
        } else {
            anyhow::bail!("Package not found")
        }
    }

    fn install_app(&self, device_id: &DeviceId, apk_data: &mut dyn std::io::Read) -> Result<()> {
        let mut buffer = Vec::new();
        apk_data.read_to_end(&mut buffer)?;

        let temp_dir = std::env::temp_dir().join("phone_backup_install");
        if !temp_dir.exists() {
            std::fs::create_dir_all(&temp_dir)?;
        }
        let temp_file = temp_dir.join(format!("{}.apk", uuid::Uuid::new_v4()));
        std::fs::write(&temp_file, buffer)?;

        let status_res = self.client.run(&["-s", &device_id.0, "install", "-r", temp_file.to_str().unwrap()]);
        let _ = std::fs::remove_file(temp_file);

        status_res?;
        Ok(())
    }
}
