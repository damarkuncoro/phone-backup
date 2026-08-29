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
                addresses: vec!["123 Rust Lane".into()],
                organizations: vec!["Ferris Corp".into()],
                notes: vec!["Met at a conference".into()],
            },
            Contact {
                name: "Jane Smith".into(),
                phones: vec!["+987654321".into()],
                emails: vec!["jane@example.com".into()],
                addresses: vec![],
                organizations: vec![],
                notes: vec![],
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
            name: Some("John Doe".into()),
            date: Utc::now(),
            duration_seconds: 120,
            type_code: 1,
            location: Some("California".into()),
        }])
    }
}
