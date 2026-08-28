use crate::client::AdbClient;
use anyhow::Result;
use domain::{CapabilityMatrix, ConnectionType, Device, DeviceId};
use ports::DevicePort;

pub struct AdbDeviceAdapter {
    client: AdbClient,
}

impl AdbDeviceAdapter {
    pub fn new() -> Self {
        Self {
            client: AdbClient::new(),
        }
    }
}

impl Default for AdbDeviceAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl DevicePort for AdbDeviceAdapter {
    fn discover(&self) -> Result<Vec<Device>> {
        let output = self.client.run(&["devices", "-l"])?;
        let mut devices = Vec::new();

        for line in output.lines().skip(1) {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 || parts[1] != "device" {
                continue;
            }

            let id = DeviceId::new(parts[0]);
            let mut model = "Unknown".to_string();

            for part in &parts[2..] {
                if part.starts_with("model:") {
                    model = part.replace("model:", "");
                }
            }

            devices.push(Device {
                id,
                manufacturer: "Unknown".into(),
                model,
                serial: parts[0].to_string(),
                os_version: "Unknown".into(),
                sdk_version: None,
                storage_total_bytes: 0,
                storage_used_bytes: 0,
                storage_free_bytes: 0,
                connection_type: if line.contains("usb:") {
                    ConnectionType::Usb
                } else {
                    ConnectionType::Wifi
                },
            });
        }

        Ok(devices)
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        let manufacturer = self
            .client
            .run(&["-s", &id.0, "shell", "getprop", "ro.product.manufacturer"])?
            .trim()
            .to_string();
        let model = self
            .client
            .run(&["-s", &id.0, "shell", "getprop", "ro.product.model"])?
            .trim()
            .to_string();
        let os_version = self
            .client
            .run(&["-s", &id.0, "shell", "getprop", "ro.build.version.release"])?
            .trim()
            .to_string();
        let sdk_version = self
            .client
            .run(&["-s", &id.0, "shell", "getprop", "ro.build.version.sdk"])?
            .trim()
            .parse()
            .ok();

        let df_output = self.client.run(&["-s", &id.0, "shell", "df", "/data"])?;
        let mut storage_total_bytes = 0;
        let mut storage_used_bytes = 0;
        let mut storage_free_bytes = 0;

        if let Some(line) = df_output.lines().nth(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                storage_total_bytes = parts[1].parse::<u64>().unwrap_or(0) * 1024;
                storage_used_bytes = parts[2].parse::<u64>().unwrap_or(0) * 1024;
                storage_free_bytes = parts[3].parse::<u64>().unwrap_or(0) * 1024;
            }
        }

        Ok(Device {
            id: id.clone(),
            manufacturer,
            model,
            serial: id.0.clone(),
            os_version,
            sdk_version,
            storage_total_bytes,
            storage_used_bytes,
            storage_free_bytes,
            connection_type: ConnectionType::Usb,
        })
    }

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        let mut matrix = CapabilityMatrix::new();
        matrix.set(domain::Capability::ReadFiles, domain::CapabilityStatus::Available);

        if self
            .client
            .run(&["-s", &id.0, "shell", "content", "query", "--uri", "content://sms", "--limit", "1"])
            .is_ok()
        {
            matrix.set(domain::Capability::ReadSms, domain::CapabilityStatus::Available);
        } else {
            matrix.set(domain::Capability::ReadSms, domain::CapabilityStatus::Denied);
        }

        if self
            .client
            .run(&[
                "-s",
                &id.0,
                "shell",
                "content",
                "query",
                "--uri",
                "content://com.android.contacts/data",
                "--limit",
                "1",
            ])
            .is_ok()
        {
            matrix.set(domain::Capability::ReadContacts, domain::CapabilityStatus::Available);
        } else {
            matrix.set(domain::Capability::ReadContacts, domain::CapabilityStatus::RequiresUserAction);
        }

        Ok(matrix)
    }

    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> {
        let child = self.client.exec_out(&id.0, &format!("cat \"{}\"", path))?;
        let stdout = child.stdout.ok_or_else(|| anyhow::anyhow!("Failed to open adb stdout"))?;
        Ok(Box::new(stdout))
    }

    fn push_file(&self, id: &DeviceId, source: &mut dyn std::io::Read, target_path: &str) -> Result<()> {
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;
        self.client.push_file(&id.0, &buffer, target_path)
    }
}
