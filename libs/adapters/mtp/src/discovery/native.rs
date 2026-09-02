use super::{MtpDiscoveryStrategy, MtpMount};
use mtp_rs::MtpDevice;
use std::path::PathBuf;
use tracing::debug;

#[derive(Default)]
pub struct NativeMtpDiscovery;

impl NativeMtpDiscovery {
    pub fn new() -> Self {
        Self
    }
}

impl MtpDiscoveryStrategy for NativeMtpDiscovery {
    fn detect(&self) -> Vec<MtpMount> {
        let mut mounts = Vec::new();

        // We only log this at debug level to avoid cluttering the terminal
        // when no devices are connected.
        debug!("NativeMtpDiscovery: Probing USB bus for MTP devices...");

        match MtpDevice::list_devices() {
            Ok(devices) => {
                for device_info in devices {
                    // Filter out known Apple internal devices by Manufacturer name
                    let manufacturer = device_info.manufacturer.as_deref().unwrap_or("Unknown");
                    if manufacturer.to_lowercase().contains("apple") {
                        continue;
                    }

                    let name = format!(
                        "{} {}",
                        manufacturer,
                        device_info.product.as_deref().unwrap_or("Device")
                    );

                    debug!(
                        "NativeMtpDiscovery: Found MTP device: {} (Serial: {})",
                        name,
                        device_info.serial_number.as_deref().unwrap_or("N/A")
                    );

                    if let Some(serial) = device_info.serial_number {
                        mounts.push(MtpMount {
                            name,
                            path: PathBuf::from(format!("usb://serial/{}", serial)),
                        });
                    } else {
                        mounts.push(MtpMount {
                            name,
                            path: PathBuf::from(format!(
                                "usb://location/{}",
                                device_info.location_id
                            )),
                        });
                    }
                }
            }
            Err(_) => {
                // Ignore errors during USB enumeration as they are usually system probes
            }
        }

        mounts
    }
}
