use anyhow::Result;
use chrono::Utc;
use domain::{CallLog, Contact, ContactEmail, ContactName, ContactPhone, DeviceId, Sms};
use ports::DataProviderPort;

pub struct MockDataProvider;

impl DataProviderPort for MockDataProvider {
    fn list_contacts(&self, _device_id: &DeviceId) -> Result<Vec<Contact>> {
        Ok(vec![
            Contact {
                id: "1".to_string(),
                snapshot_id: None,
                source_id: Some("mock_1".to_string()),
                display_name: "John Doe".into(),
                notes: Some("Met at a conference".into()),
                source: "mock".to_string(),
                source_account: None,
                content_hash: None,
                metadata_json: None,
                names: vec![ContactName {
                    display_name: Some("John Doe".into()),
                    given_name: Some("John".into()),
                    family_name: Some("Doe".into()),
                    middle_name: None,
                    prefix: None,
                    suffix: None,
                }],
                phones: vec![ContactPhone {
                    raw_value: "+123456789".into(),
                    normalized_value: Some("+123456789".into()),
                    phone_type: Some("mobile".into()),
                    label: None,
                    is_primary: true,
                }],
                emails: vec![ContactEmail {
                    value: "john@example.com".into(),
                    email_type: Some("work".into()),
                    label: None,
                    is_primary: true,
                }],
                addresses: vec![],
                organizations: vec![],
                urls: vec![],
                events: vec![],
                photos: vec![],
                labels: vec![],
            },
            Contact {
                id: "2".to_string(),
                snapshot_id: None,
                source_id: Some("mock_2".to_string()),
                display_name: "Jane Smith".into(),
                notes: None,
                source: "mock".to_string(),
                source_account: None,
                content_hash: None,
                metadata_json: None,
                names: vec![],
                phones: vec![ContactPhone {
                    raw_value: "+987654321".into(),
                    normalized_value: Some("+987654321".into()),
                    phone_type: Some("home".into()),
                    label: None,
                    is_primary: true,
                }],
                emails: vec![],
                addresses: vec![],
                organizations: vec![],
                urls: vec![],
                events: vec![],
                photos: vec![],
                labels: vec![],
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
