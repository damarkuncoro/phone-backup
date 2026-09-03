use domain::{Capability, CapabilityMatrix, CapabilityStatus, ConnectionType, Device, DeviceId};
use serde::{Deserialize, Serialize};

/// iOS Device Information parsed from Apple Lockdown / Plist protocol.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IosDeviceProperties {
    pub unique_device_id: String,
    pub device_name: String,
    pub product_type: String,
    pub product_version: String,
    pub serial_number: Option<String>,
    pub total_disk_capacity: Option<u64>,
    pub total_data_available: Option<u64>,
}

impl IosDeviceProperties {
    /// Maps Apple internal hardware identifier to user-friendly marketing name.
    pub fn get_marketing_name(&self) -> String {
        match self.product_type.as_str() {
            "iPhone14,2" => "iPhone 13 Pro".to_string(),
            "iPhone14,3" => "iPhone 13 Pro Max".to_string(),
            "iPhone14,5" => "iPhone 13".to_string(),
            "iPhone15,2" => "iPhone 14 Pro".to_string(),
            "iPhone15,3" => "iPhone 14 Pro Max".to_string(),
            "iPhone15,4" => "iPhone 15".to_string(),
            "iPhone15,5" => "iPhone 15 Plus".to_string(),
            "iPhone16,1" => "iPhone 15 Pro".to_string(),
            "iPhone16,2" => "iPhone 15 Pro Max".to_string(),
            "iPhone17,1" => "iPhone 16 Pro".to_string(),
            "iPhone17,2" => "iPhone 16 Pro Max".to_string(),
            other if other.starts_with("iPhone") => format!("Apple {}", other),
            other if other.starts_with("iPad") => format!("Apple {}", other),
            _ => self.device_name.clone(),
        }
    }

    /// Converts raw iOS metadata into core domain Device.
    pub fn to_device(&self) -> Device {
        let total = self.total_disk_capacity.unwrap_or(128 * 1024 * 1024 * 1024);
        let free = self.total_data_available.unwrap_or(64 * 1024 * 1024 * 1024);
        let used = total.saturating_sub(free);

        Device {
            id: DeviceId::new(&self.unique_device_id),
            manufacturer: "Apple".to_string(),
            model: self.get_marketing_name(),
            serial: self.serial_number.clone().unwrap_or_else(|| self.unique_device_id.clone()),
            os_version: format!("iOS {}", self.product_version),
            sdk_version: None,
            storage_total_bytes: total,
            storage_used_bytes: used,
            storage_free_bytes: free,
            connection_type: ConnectionType::Usb,
        }
    }

    /// Builds capability matrix for an iOS device.
    pub fn to_capability_matrix(&self) -> CapabilityMatrix {
        let mut matrix = CapabilityMatrix::new();
        matrix.set(Capability::ReadFiles, CapabilityStatus::Available);
        matrix.set(Capability::ReadMedia, CapabilityStatus::Available);
        matrix.set(Capability::ReadDownload, CapabilityStatus::Available);
        matrix.set(Capability::ReadDocuments, CapabilityStatus::Available);
        matrix.set(Capability::ReadContacts, CapabilityStatus::Available);
        matrix.set(Capability::ReadSms, CapabilityStatus::Unsupported);
        matrix.set(Capability::ReadCallLog, CapabilityStatus::Unsupported);
        matrix.set(Capability::ReadAppData, CapabilityStatus::Unsupported);
        matrix
    }
}
