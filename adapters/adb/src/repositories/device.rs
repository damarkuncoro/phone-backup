use crate::client::AdbClient;
use crate::parsers::device_parser::DeviceParser;
use crate::parsers::battery_parser::BatteryParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::{CapabilityMatrix, Device, DeviceId, ConnectionType};
use std::io::Read;

#[derive(Clone)]
pub struct AdbDeviceRepository {
    client: AdbClient,
}

impl AdbDeviceRepository {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    pub fn discover(&self) -> Result<Vec<Device>> {
        let output = self.client.run(&["devices", "-l"])?;
        Ok(DeviceParser::parse_devices_l(&output))
    }

    pub fn get_info(&self, id: &DeviceId) -> Result<Device> {
        let manufacturer = self.client.get_prop(&id.0, "ro.product.manufacturer")?;
        let model = self.client.get_prop(&id.0, "ro.product.model")?;
        let os_version = self.client.get_prop(&id.0, "ro.build.version.release")?;
        let sdk_version = self.client.get_prop(&id.0, "ro.build.version.sdk")?.parse().ok();

        let df_output = self.client.shell(&id.0, AndroidScripts::DISK_USAGE)?;
        let (total, used, free) = DeviceParser::parse_df_output(&df_output);

        Ok(Device {
            id: id.clone(),
            manufacturer,
            model,
            serial: id.0.clone(),
            os_version,
            sdk_version,
            storage_total_bytes: total,
            storage_used_bytes: used,
            storage_free_bytes: free,
            connection_type: ConnectionType::Usb,
        })
    }

    pub fn get_capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        let mut matrix = CapabilityMatrix::new();
        matrix.set(domain::Capability::ReadFiles, domain::CapabilityStatus::Available);

        let sms_check = self.client.shell(&id.0, &AndroidScripts::content_query("content://sms", "address"));
        if sms_check.is_ok() && !sms_check.unwrap().contains("Error") {
            matrix.set(domain::Capability::ReadSms, domain::CapabilityStatus::Available);
        } else {
            matrix.set(domain::Capability::ReadSms, domain::CapabilityStatus::Denied);
        }

        let contacts_check = self.client.shell(&id.0, &AndroidScripts::content_query("content://com.android.contacts/data", "contact_id"));
        if contacts_check.is_ok() && !contacts_check.unwrap().contains("Error") {
            matrix.set(domain::Capability::ReadContacts, domain::CapabilityStatus::Available);
        } else {
            matrix.set(domain::Capability::ReadContacts, domain::CapabilityStatus::RequiresUserAction);
        }

        Ok(matrix)
    }

    pub fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn Read>> {
        self.client.stream_file(&id.0, path)
    }

    pub fn push_file(&self, id: &DeviceId, source: &mut dyn Read, target_path: &str) -> Result<()> {
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;
        self.client.push_file(&id.0, &buffer, target_path)
    }

    pub fn get_battery_status(&self, id: &DeviceId) -> Result<(u32, f32)> {
        let output = self.client.shell(&id.0, AndroidScripts::BATTERY_STATUS)?;
        if let Some(status) = BatteryParser::parse(&output) {
            Ok((status.level, status.temperature))
        } else {
            anyhow::bail!("Failed to parse battery status")
        }
    }
}
