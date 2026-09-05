use domain::{AppId, AppInfo, DeviceId};
use std::collections::HashMap;

pub struct AppParser;

impl AppParser {
    /// Parse output from 'pm list packages --show-versioncode' and merge with dumpsys versions
    pub fn parse_pm_list_detailed(
        device_id: &DeviceId,
        pm_output: &str,
        version_map: &HashMap<String, String>,
    ) -> Vec<AppInfo> {
        let mut apps = Vec::new();

        for line in pm_output.lines() {
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

                let version_name = version_map
                    .get(pkg_name)
                    .cloned()
                    .unwrap_or_else(|| "1.0".to_string());

                let app_name = Self::derive_friendly_name(pkg_name);

                apps.push(AppInfo {
                    id: AppId(pkg_name.to_string()),
                    device_id: device_id.clone(),
                    package_name: pkg_name.to_string(),
                    version_name,
                    version_code,
                    installer: None,
                    app_name,
                });
            }
        }
        apps
    }

    /// Parse package and versionName pairs from dumpsys package output
    pub fn parse_dumpsys_versions(dumpsys_output: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let mut current_pkg: Option<String> = None;

        for line in dumpsys_output.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Package [") {
                if let Some(start) = trimmed.find('[') {
                    if let Some(end) = trimmed.find(']') {
                        current_pkg = Some(trimmed[start + 1..end].to_string());
                    }
                }
            } else if trimmed.starts_with("versionName=") {
                if let Some(pkg) = &current_pkg {
                    let version = trimmed.replace("versionName=", "");
                    map.insert(pkg.clone(), version);
                }
            }
        }
        map
    }

    /// Derive human-friendly display name from common Android package names
    pub fn derive_friendly_name(pkg: &str) -> String {
        match pkg {
            "com.whatsapp" => "WhatsApp".to_string(),
            "com.whatsapp.w4b" => "WhatsApp Business".to_string(),
            "com.google.android.youtube" => "YouTube".to_string(),
            "com.google.android.apps.maps" => "Google Maps".to_string(),
            "com.google.android.apps.docs" => "Google Docs".to_string(),
            "com.google.android.apps.photos" => "Google Photos".to_string(),
            "com.android.chrome" => "Google Chrome".to_string(),
            "com.spotify.music" => "Spotify".to_string(),
            "com.instagram.android" => "Instagram".to_string(),
            "org.telegram.messenger" => "Telegram".to_string(),
            _ => {
                // If package has dots like com.vivo.soundrecorder -> Soundrecorder
                if let Some(last_segment) = pkg.split('.').next_back() {
                    let mut c = last_segment.chars();
                    match c.next() {
                        None => pkg.to_string(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                } else {
                    pkg.to_string()
                }
            }
        }
    }
}
