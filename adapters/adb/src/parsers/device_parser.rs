use domain::{ConnectionType, Device, DeviceId};

pub struct DeviceParser;

impl DeviceParser {
    pub fn parse_devices_l(output: &str) -> Vec<Device> {
        let mut devices = Vec::new();

        for line in output.lines().skip(1) {
            if line.is_empty() { continue; }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 || parts[1] != "device" {
                continue;
            }

            let serial = parts[0];
            let id = DeviceId::new(serial);
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
                serial: serial.to_string(),
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

        devices
    }

    pub fn parse_df_output(output: &str) -> (u64, u64, u64) {
        if let Some(line) = output.lines().nth(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let total = parts[1].parse::<u64>().unwrap_or(0) * 1024;
                let used = parts[2].parse::<u64>().unwrap_or(0) * 1024;
                let free = parts[3].parse::<u64>().unwrap_or(0) * 1024;
                return (total, used, free);
            }
        }
        (0, 0, 0)
    }
}
