use serde::{Deserialize, Serialize};

/// Stable identifier for a device (e.g. ADB serial, MTP GUID).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.pad (not write!) so width/alignment flags from callers like
        // `{:<15}` in the CLI table are actually honored.
        f.pad(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    Usb,
    Wifi,
    Mtp,
    Unknown,
}

/// A phone/tablet known to the backup platform.
///
/// This struct intentionally has no notion of *how* it was discovered
/// (ADB vs MTP vs future iOS adapter) — that belongs to the adapter layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: DeviceId,
    pub manufacturer: String,
    pub model: String,
    pub serial: String,
    pub os_version: String,
    pub sdk_version: Option<u32>,
    pub storage_total_bytes: u64,
    pub storage_used_bytes: u64,
    pub storage_free_bytes: u64,
    pub connection_type: ConnectionType,
}

impl Device {
    pub fn storage_used_percent(&self) -> f32 {
        if self.storage_total_bytes == 0 {
            return 0.0;
        }
        (self.storage_used_bytes as f32 / self.storage_total_bytes as f32) * 100.0
    }
}
