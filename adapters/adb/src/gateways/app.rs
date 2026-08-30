use crate::client::AdbClient;
use crate::parsers::app_parser::AppParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::DeviceId;
use ports::AppProviderPort;

pub struct AdbAppGateway {
    client: AdbClient,
}

impl AdbAppGateway {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }
}

impl AppProviderPort for AdbAppGateway {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<domain::AppInfo>> {
        let stdout = self.client.shell(&device_id.0, AndroidScripts::LIST_APPS)?;
        Ok(AppParser::parse_pm_list_detailed(device_id, &stdout))
    }

    fn get_apk(&self, device_id: &DeviceId, package_name: &str) -> Result<Box<dyn std::io::Read>> {
        let stdout = self.client.shell(&device_id.0, &format!("pm path {}", package_name))?;
        if let Some(path) = stdout.lines().next().and_then(|l| l.strip_prefix("package:")) {
            self.client.stream_file(&device_id.0, path)
        } else {
            anyhow::bail!("Package not found: {}", package_name)
        }
    }

    fn install_app(&self, device_id: &DeviceId, apk_data: &mut dyn std::io::Read) -> Result<()> {
        let mut buffer = Vec::new();
        apk_data.read_to_end(&mut buffer)?;
        let remote_path = "/data/local/tmp/temp_install.apk";
        self.client.push_file(&device_id.0, &buffer, remote_path)?;

        let res = self.client.shell(&device_id.0, &format!("pm install -r {}", remote_path));
        let _ = self.client.shell(&device_id.0, &format!("rm {}", remote_path));

        res.map(|_| ())
    }
}
