use crate::repositories::AdbDataRepository;
use anyhow::Result;
use domain::{CallLog, Contact, DeviceId, Sms};
use ports::DataProviderPort;

#[derive(Clone)]
pub struct AdbDataGateway {
    repo: AdbDataRepository,
}

impl AdbDataGateway {
    pub fn new(repo: AdbDataRepository) -> Self {
        Self { repo }
    }
}

impl DataProviderPort for AdbDataGateway {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>> {
        self.repo.list_contacts(device_id)
    }

    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>> {
        self.repo.list_sms(device_id)
    }

    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>> {
        self.repo.list_call_logs(device_id)
    }
}
