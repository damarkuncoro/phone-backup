use super::AgentAdapter;
use anyhow::Result;
use domain::{AppId, AppInfo, Contact, DeviceId};
use ports::{AppProviderPort, DataProviderPort};

impl DataProviderPort for AgentAdapter {
    fn list_contacts(&self, _device_id: &DeviceId) -> Result<Vec<Contact>> {
        let data = self.session.structured_data.read().unwrap();
        if !data.contacts.is_empty() {
            return Ok(data.contacts.clone());
        }

        Ok(vec![Contact {
            id: "agent_c1".to_string(),
            snapshot_id: None,
            source_id: Some("agent_src_1".to_string()),
            display_name: "Damar Kuncoro (Wireless)".into(),
            notes: Some("Synced via Companion Agent".into()),
            source: "companion_agent".to_string(),
            source_account: None,
            content_hash: None,
            metadata_json: None,
            names: vec![domain::ContactName {
                display_name: Some("Damar Kuncoro (Wireless)".into()),
                given_name: Some("Damar".into()),
                family_name: Some("Kuncoro".into()),
                middle_name: None,
                prefix: None,
                suffix: None,
            }],
            phones: vec![domain::ContactPhone {
                raw_value: "+6285921495599".into(),
                normalized_value: Some("+6285921495599".into()),
                phone_type: Some("mobile".into()),
                label: None,
                is_primary: true,
            }],
            emails: vec![domain::ContactEmail {
                value: "damar@example.com".into(),
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
        }])
    }

    fn list_sms(&self, _device_id: &DeviceId) -> Result<Vec<domain::Sms>> {
        let data = self.session.structured_data.read().unwrap();
        Ok(data.sms.clone())
    }

    fn list_call_logs(&self, _device_id: &DeviceId) -> Result<Vec<domain::CallLog>> {
        let data = self.session.structured_data.read().unwrap();
        Ok(data.call_logs.clone())
    }
}

impl AppProviderPort for AgentAdapter {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> {
        let data = self.session.structured_data.read().unwrap();
        if !data.apps.is_empty() {
            return Ok(data.apps.clone());
        }

        Ok(vec![AppInfo {
            id: AppId("com.phonebackup.agent".into()),
            device_id: device_id.clone(),
            package_name: "com.phonebackup.agent".into(),
            version_name: "1.0.0".into(),
            version_code: 1,
            installer: Some("com.android.vending".into()),
            app_name: "Phone Backup Companion Agent".into(),
        }])
    }

    fn get_apk(
        &self,
        _device_id: &DeviceId,
        _package_name: &str,
    ) -> Result<Box<dyn std::io::Read>> {
        let content = vec![0u8; 1024];
        Ok(Box::new(std::io::Cursor::new(content)))
    }

    fn install_app(&self, _device_id: &DeviceId, _apk_data: &mut dyn std::io::Read) -> Result<()> {
        Ok(())
    }
}
