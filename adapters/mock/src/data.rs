use anyhow::Result;
use chrono::Utc;
use domain::{CallLog, Contact, DeviceId, Sms};
use ports::DataProviderPort;

pub struct MockDataProvider;

impl DataProviderPort for MockDataProvider {
    fn list_contacts(&self, _device_id: &DeviceId) -> Result<Vec<Contact>> {
        Ok(vec![
            Contact {
                name: "John Doe".into(),
                phones: vec!["+123456789".into()],
                emails: vec!["john@example.com".into()],
            },
            Contact {
                name: "Jane Smith".into(),
                phones: vec!["+987654321".into()],
                emails: vec!["jane@example.com".into()],
            },
        ])
    }

    fn list_sms(&self, _device_id: &DeviceId) -> Result<Vec<Sms>> {
        Ok(vec![Sms {
            address: "+123456789".into(),
            body: "Hello, this is a test SMS".into(),
            date: Utc::now(),
            type_code: 1,
        }])
    }

    fn list_call_logs(&self, _device_id: &DeviceId) -> Result<Vec<CallLog>> {
        Ok(vec![CallLog {
            number: "+123456789".into(),
            date: Utc::now(),
            duration_seconds: 120,
            type_code: 1,
        }])
    }
}
