use domain::{App, CallLog, Contact, Device, DeviceId, DeviceInfo, FileEntry, Sms};
use serde::{Deserialize, Serialize};

/// Handshake payload sent by the Android Companion Agent upon connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHandshake {
    pub device_id: String,
    pub manufacturer: String,
    pub model: String,
    pub android_version: String,
    pub storage_used_bytes: u64,
    pub storage_total_bytes: u64,
    pub capabilities: Vec<String>,
    pub battery_percent: Option<u8>,
    pub temperature_c: Option<f32>,
}

impl AgentHandshake {
    pub fn to_device(&self) -> Device {
        Device::new(
            DeviceId(self.device_id.clone()),
            self.model.clone(),
            self.android_version.clone(),
        )
    }

    pub fn to_device_info(&self) -> DeviceInfo {
        DeviceInfo {
            id: DeviceId(self.device_id.clone()),
            manufacturer: self.manufacturer.clone(),
            model: self.model.clone(),
            android_version: self.android_version.clone(),
            storage_used_bytes: self.storage_used_bytes,
            storage_total_bytes: self.storage_total_bytes,
            capabilities: domain::CapabilityMatrix::full_access(),
        }
    }
}

/// Remote file list response from the Android Companion Agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentFileScanResponse {
    pub files: Vec<FileEntry>,
}

/// Structured data response containing Contacts, SMS, and Call Logs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentStructuredDataResponse {
    pub contacts: Vec<Contact>,
    pub sms: Vec<Sms>,
    pub call_logs: Vec<CallLog>,
    pub apps: Vec<App>,
}

/// Real-time status update from the Android Companion Agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHeartbeat {
    pub battery_percent: u8,
    pub temperature_c: f32,
    pub is_charging: bool,
}
