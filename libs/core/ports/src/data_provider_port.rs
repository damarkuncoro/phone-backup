use anyhow::Result;
use domain::{CallLog, Contact, DeviceId, Sms};

pub trait DataProviderPort: Send + Sync {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>>;
    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>>;
    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>>;

    fn restore_contacts(&self, _device_id: &DeviceId, _contacts: &[Contact]) -> Result<usize> {
        anyhow::bail!("Contact restore not supported by this provider")
    }
}
