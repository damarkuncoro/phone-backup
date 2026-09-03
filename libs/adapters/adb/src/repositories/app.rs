use crate::client::AdbClient;
use crate::parsers::app_parser::AppParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::{AppInfo, DeviceId};
use std::io::Read;

#[derive(Clone)]
pub struct AdbAppRepository {
    client: AdbClient,
}

impl AdbAppRepository {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    pub fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> {
        let stdout = self.client.shell(&device_id.0, AndroidScripts::LIST_APPS)?;
        let version_map = if let Ok(dumpsys) = self.client.shell(&device_id.0, "dumpsys package packages | grep -E 'Package \\[|versionName='") {
            AppParser::parse_dumpsys_versions(&dumpsys)
        } else {
            std::collections::HashMap::new()
        };
        Ok(AppParser::parse_pm_list_detailed(device_id, &stdout, &version_map))
    }

    pub fn get_apk(&self, device_id: &DeviceId, package_name: &str) -> Result<Box<dyn Read>> {
        let stdout = self
            .client
            .shell(&device_id.0, &format!("pm path {}", package_name))?;
        if let Some(path) = stdout
            .lines()
            .next()
            .and_then(|l| l.strip_prefix("package:"))
        {
            self.client.stream_file(&device_id.0, path)
        } else {
            anyhow::bail!("Package not found: {}", package_name)
        }
    }

    pub fn install_app(&self, device_id: &DeviceId, apk_data: &mut dyn Read) -> Result<()> {
        let mut buffer = Vec::new();
        apk_data.read_to_end(&mut buffer)?;
        let remote_path = "/data/local/tmp/temp_install.apk";
        self.client.push_file(&device_id.0, &buffer, remote_path)?;

        let res = self
            .client
            .shell(&device_id.0, &format!("pm install -r {}", remote_path));
        let _ = self
            .client
            .shell(&device_id.0, &format!("rm {}", remote_path));

        res.map(|_| ())
    }

    pub fn install_split_bundle(
        &self,
        device_id: &DeviceId,
        splits: &[(&str, &[u8])],
    ) -> Result<()> {
        if splits.is_empty() {
            anyhow::bail!("No APK splits provided for installation");
        }

        let create_out = self.client.shell(&device_id.0, "pm install-create -r")?;
        let session_id = create_out
            .split('[')
            .nth(1)
            .and_then(|s| s.split(']').next())
            .map(|s| s.trim())
            .ok_or_else(|| anyhow::anyhow!("Failed to parse session ID from: {}", create_out))?;

        for (split_name, split_bytes) in splits {
            let clean_name = split_name.trim_end_matches(".apk");
            let remote_tmp = format!("/data/local/tmp/{}_{}.apk", session_id, clean_name);
            if let Err(e) = self.client.push_file(&device_id.0, split_bytes, &remote_tmp) {
                let _ = self.client.shell(&device_id.0, &format!("pm install-abandon {}", session_id));
                anyhow::bail!("Failed to push split APK {}: {}", split_name, e);
            }

            let write_cmd = format!("pm install-write {} {} {}", session_id, clean_name, remote_tmp);
            let write_res = self.client.shell(&device_id.0, &write_cmd);
            let _ = self.client.shell(&device_id.0, &format!("rm {}", remote_tmp));

            if let Err(e) = write_res {
                let _ = self.client.shell(&device_id.0, &format!("pm install-abandon {}", session_id));
                anyhow::bail!("Failed to write split APK {} to session: {}", split_name, e);
            }
        }

        let commit_res = self.client.shell(&device_id.0, &format!("pm install-commit {}", session_id))?;
        if commit_res.contains("Success") {
            Ok(())
        } else {
            let _ = self.client.shell(&device_id.0, &format!("pm install-abandon {}", session_id));
            anyhow::bail!("Session commit failed: {}", commit_res.trim())
        }
    }
}
