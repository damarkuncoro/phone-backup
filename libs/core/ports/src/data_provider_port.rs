use anyhow::Result;
use domain::{CallLog, Contact, DeviceId, Sms};

pub trait DataProviderPort: Send + Sync {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>>;
    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>>;
    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>>;
}
