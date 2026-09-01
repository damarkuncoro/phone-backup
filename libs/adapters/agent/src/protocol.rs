use domain::{AppInfo, Capability, CapabilityMatrix, CapabilityStatus, ConnectionType, Contact, Device, DeviceId, FileEntry, Sms};
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
    pub battery_percent: Option<u8>,
    pub temperature_c: Option<f32>,
}

impl AgentHandshake {
    pub fn to_device(&self) -> Device {
        let free = self.storage_total_bytes.saturating_sub(self.storage_used_bytes);
        Device {
            id: DeviceId::new(self.device_id.clone()),
            manufacturer: self.manufacturer.clone(),
            model: self.model.clone(),
            serial: self.device_id.clone(),
            os_version: self.android_version.clone(),
            sdk_version: Some(34),
            storage_total_bytes: self.storage_total_bytes,
            storage_used_bytes: self.storage_used_bytes,
            storage_free_bytes: free,
            connection_type: ConnectionType::Wifi,
        }
    }

    pub fn to_capability_matrix(&self) -> CapabilityMatrix {
        let mut matrix = CapabilityMatrix::new();
        matrix.set(Capability::ReadFiles, CapabilityStatus::Available);
        matrix.set(Capability::ReadMedia, CapabilityStatus::Available);
        matrix.set(Capability::ReadDownload, CapabilityStatus::Available);
        matrix.set(Capability::ReadDocuments, CapabilityStatus::Available);
        matrix.set(Capability::ReadAppData, CapabilityStatus::Available);
        matrix.set(Capability::ReadContacts, CapabilityStatus::Available);
        matrix.set(Capability::ReadSms, CapabilityStatus::Available);
        matrix.set(Capability::ReadCallLog, CapabilityStatus::Available);
        matrix
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
    pub call_logs: Vec<domain::CallLog>,
    pub apps: Vec<AppInfo>,
}

/// Real-time status update from the Android Companion Agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHeartbeat {
    pub battery_percent: u8,
    pub temperature_c: f32,
    pub is_charging: bool,
}
