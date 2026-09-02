use anyhow::{Context, Result};
use domain::{ConnectionType, Device, DeviceId};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tracing::{error, info, warn};

pub enum DeviceEvent {
    Connected(Device),
    Disconnected(DeviceId),
}

pub struct AdbMonitor {
    adb_path: String,
}

impl AdbMonitor {
    pub fn new(adb_path: String) -> Self {
        Self { adb_path }
    }

    pub fn track_devices<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(DeviceEvent),
    {
        loop {
            info!("Starting ADB device monitor via track-devices...");

            let mut child = Command::new(&self.adb_path)
                .arg("track-devices")
                .stdout(Stdio::piped())
                .spawn()
                .context("Failed to start adb track-devices")?;

            let stdout = child
                .stdout
                .take()
                .context("Failed to capture adb stdout")?;
            let reader = BufReader::new(stdout);

            // Track currently known devices to detect disconnects
            let mut known_devices: std::collections::HashSet<String> =
                std::collections::HashSet::new();

            for line in reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(e) => {
                        error!("Error reading adb track-devices output: {}", e);
                        break;
                    }
                };

                if line.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    continue;
                }

                let serial = parts[0].to_string();
                let state = parts[1];

                if state == "device" {
                    if !known_devices.contains(&serial) {
                        known_devices.insert(serial.clone());
                        let device = Device {
                            id: DeviceId::new(&serial),
                            manufacturer: "Unknown".into(),
                            model: "Unknown".into(),
                            serial: serial.clone(),
                            os_version: "Unknown".into(),
                            sdk_version: None,
                            storage_total_bytes: 0,
                            storage_used_bytes: 0,
                            storage_free_bytes: 0,
                            connection_type: ConnectionType::Usb,
                        };
                        callback(DeviceEvent::Connected(device));
                    }
                } else {
                    if known_devices.remove(&serial) {
                        callback(DeviceEvent::Disconnected(DeviceId::new(&serial)));
                    }
                }
            }

            // Cleanup any remaining known devices if monitor crashed
            for serial in known_devices.drain() {
                callback(DeviceEvent::Disconnected(DeviceId::new(&serial)));
            }

            let _ = child.wait();
            warn!("ADB track-devices process exited. Restarting in 2 seconds...");
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}
