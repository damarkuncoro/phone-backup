use anyhow::{anyhow, Result};
use mtp_rs::MtpDevice;
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Clone)]
pub struct NativeMtpOperations {
    pub(crate) serial: Option<String>,
    pub(crate) location_id: Option<u64>,
    pub(crate) device_cache: Arc<Mutex<Option<MtpDevice>>>,
}

impl NativeMtpOperations {
    pub fn new_from_serial(serial: String) -> Result<Self> {
        Ok(Self {
            serial: Some(serial),
            location_id: None,
            device_cache: Arc::new(Mutex::new(None)),
        })
    }

    pub fn new_from_location(loc: u64) -> Result<Self> {
        Ok(Self {
            serial: None,
            location_id: Some(loc),
            device_cache: Arc::new(Mutex::new(None)),
        })
    }

    pub(crate) async fn get_device(&self) -> Result<MtpDevice> {
        {
            let cache = self.device_cache.lock().unwrap();
            if let Some(ref dev) = *cache {
                return Ok(dev.clone());
            }
        }

        if let Some(ref s) = self.serial {
            let _ = crate::resolver::MtpConflictResolver::resolve_conflicts(s);
        } else {
            let _ = crate::resolver::MtpConflictResolver::kill_conflicts();
        }

        let mut last_error = anyhow!("Unknown error");
        for attempt in 1..=4 {
            let result = self.open_device_attempt().await;
            match result {
                Ok(dev) => {
                    info!("MTP: Successfully opened persistent session");
                    let mut cache = self.device_cache.lock().unwrap();
                    *cache = Some(dev.clone());
                    return Ok(dev);
                }
                Err(e) => {
                    let err_msg = e.to_string();
                    last_error = anyhow!(err_msg.clone());

                    info!(
                        "MTP: Attempt {} failed ({}). Waiting to retry...",
                        attempt, err_msg
                    );
                    let _ = std::process::Command::new("killall")
                        .args(["-9", "PTPCamera", "ptpcamera", "ptpcamerad"])
                        .output();

                    if err_msg.contains("Transaction ID mismatch") {
                        if let Some(ref s) = self.serial {
                            let _ = MtpDevice::reset_by_serial(s).await;
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                }
            }
        }

        anyhow::bail!(
            "Gagal membuka koneksi ke HP: {}. TIPS: Cabut dan colok kembali kabel USB Anda.",
            last_error
        )
    }

    async fn open_device_attempt(&self) -> Result<MtpDevice> {
        if let Some(ref s) = self.serial {
            match MtpDevice::open_by_serial(s).await {
                Ok(d) => Ok(d),
                Err(_) => {
                    if let Ok(devices) = MtpDevice::list_devices() {
                        if let Some(target) = devices.iter().find(|d| d.serial_number.as_deref() == Some(s)) {
                            MtpDevice::open_by_location(target.location_id).await.map_err(|e| anyhow!(e))
                        } else if let Some(target) = devices.iter().find(|d| {
                            !d.manufacturer.as_deref().unwrap_or("").to_lowercase().contains("apple")
                        }) {
                            MtpDevice::open_by_location(target.location_id).await.map_err(|e| anyhow!(e))
                        } else {
                            Err(anyhow!("No MTP device found matching serial {}", s))
                        }
                    } else {
                        Err(anyhow!("Failed to enumerate MTP devices"))
                    }
                }
            }
        } else if let Some(loc) = self.location_id {
            MtpDevice::open_by_location(loc).await.map_err(|e| anyhow!(e))
        } else {
            anyhow::bail!("No identification provided for MTP device")
        }
    }

    pub fn get_storage_info(&self) -> Result<(u64, u64)> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        rt.block_on(async {
            let device = self.get_device().await?;
            let storages = device.storages().await?;
            let mut total = 0u64;
            let mut free = 0u64;
            for s in storages {
                total += s.info().total_capacity;
                free += s.info().free_space;
            }
            if total == 0 {
                total = 64 * 1024 * 1024 * 1024;
                free = 20 * 1024 * 1024 * 1024;
            }
            Ok((total, free))
        })
    }
}
