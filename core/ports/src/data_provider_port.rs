use anyhow::Result;
use domain::{DeviceId, Contact, Sms, CallLog};

pub trait DataProviderPort {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>>;
    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>>;
    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>>;
}
