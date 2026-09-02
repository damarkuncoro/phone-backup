use domain::{AppId, AppInfo, DeviceId};

pub struct AppParser;

impl AppParser {
    /// Parse output from 'pm list packages --show-versioncode'
    pub fn parse_pm_list_detailed(device_id: &DeviceId, output: &str) -> Vec<AppInfo> {
        let mut apps = Vec::new();

        for line in output.lines() {
            // Format: package:com.example versionCode:123
            if let Some(stripped) = line.strip_prefix("package:") {
                let parts: Vec<&str> = stripped.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                let pkg_name = parts[0];
                let mut version_code = 0;

                for part in &parts[1..] {
                    if part.starts_with("versionCode:") {
                        version_code = part.replace("versionCode:", "").parse().unwrap_or(0);
                    }
                }

                apps.push(AppInfo {
                    id: AppId(pkg_name.to_string()),
                    device_id: device_id.clone(),
                    package_name: pkg_name.to_string(),
                    version_name: "Unknown".into(),
                    version_code,
                    installer: None,
                    app_name: pkg_name.to_string(), // Fallback to pkg name
                });
            }
        }
        apps
    }
}
