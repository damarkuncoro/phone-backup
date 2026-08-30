use crate::client::AdbClient;
use crate::parsers::device_parser::DeviceParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::{CapabilityMatrix, Device, DeviceId, ConnectionType};
use ports::DevicePort;

pub struct AdbDeviceGateway {
    client: AdbClient,
}

impl AdbDeviceGateway {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }
}

impl DevicePort for AdbDeviceGateway {
    fn discover(&self) -> Result<Vec<Device>> {
        let output = self.client.run(&["devices", "-l"])?;
        Ok(DeviceParser::parse_devices_l(&output))
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
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

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
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

    fn read_file(&self, id: &DeviceId, path: &str) -> Result<Box<dyn std::io::Read>> {
        let child = self.client.exec_out(&id.0, &AndroidScripts::cat_file(path))?;
        let stdout = child.stdout.ok_or_else(|| anyhow::anyhow!("Failed to open adb stdout"))?;
        Ok(Box::new(stdout))
    }

    fn push_file(&self, id: &DeviceId, source: &mut dyn std::io::Read, target_path: &str) -> Result<()> {
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;
        self.client.push_file(&id.0, &buffer, target_path)
    }
}
