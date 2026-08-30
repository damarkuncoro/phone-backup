use std::sync::{Arc, RwLock};
use anyhow::{bail, Result};
use domain::{
    App, CallLog, CapabilityMatrix, Contact, Device, DeviceId, DomainError, FileEntry,
};
use ports::{AppProviderPort, DataProviderPort, DevicePort, ScannerPort};
use crate::protocol::{AgentFileScanResponse, AgentHandshake, AgentStructuredDataResponse};

/// In-memory state and registry of active Android Companion Agent sessions.
#[derive(Clone, Default)]
pub struct AgentSessionManager {
    devices: Arc<RwLock<Vec<AgentHandshake>>>,
    scanned_files: Arc<RwLock<Vec<FileEntry>>>,
    structured_data: Arc<RwLock<AgentStructuredDataResponse>>,
}

impl AgentSessionManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a newly connected Android Companion Agent.
    pub fn register_device(&self, handshake: AgentHandshake) {
        let mut devs = self.devices.write().unwrap();
        devs.retain(|d| d.device_id != handshake.device_id);
        devs.push(handshake);
    }

    /// Update cached file manifest for the connected agent.
    pub fn update_files(&self, files: Vec<FileEntry>) {
        let mut f = self.scanned_files.write().unwrap();
        *f = files;
    }

    /// Update structured data (Contacts, SMS, Call Logs) from the agent.
    pub fn update_structured_data(&self, data: AgentStructuredDataResponse) {
        let mut s = self.structured_data.write().unwrap();
        *s = data;
    }
}

/// Agent adapter implementing Hexagonal Ports for wireless Android Agent communication.
#[derive(Clone)]
pub struct AgentAdapter {
    session: AgentSessionManager,
}

impl AgentAdapter {
    pub fn new(session: AgentSessionManager) -> Self {
        Self { session }
    }

    pub fn with_default_session() -> Self {
        let session = AgentSessionManager::new();
        // Seed default wireless mock device for testing / standalone CLI mode
        session.register_device(AgentHandshake {
            device_id: "AGENT_WIRELESS_01".to_string(),
            manufacturer: "Xiaomi".to_string(),
            model: "22101316G (Wireless Agent)".to_string(),
            android_version: "Android 14 (HyperOS)".to_string(),
            storage_used_bytes: 41_481_015_296,
            storage_total_bytes: 242_017_599_488,
            capabilities: vec!["ReadFiles".into(), "ReadContacts".into(), "ReadSms".into()],
            battery_percent: Some(95),
            temperature_c: Some(34.5),
        });
        Self { session }
    }
}

impl Default for AgentAdapter {
    fn default() -> Self {
        Self::with_default_session()
    }
}

impl DevicePort for AgentAdapter {
    fn discover(&self) -> Result<Vec<Device>> {
        let devs = self.session.devices.read().unwrap();
        Ok(devs.iter().map(|d| d.to_device()).collect())
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        let devs = self.session.devices.read().unwrap();
        devs.iter()
            .find(|d| d.device_id == id.0)
            .map(|d| d.to_device())
            .ok_or_else(|| anyhow::anyhow!(DomainError::DeviceNotFound(id.to_string())))
    }

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        let devs = self.session.devices.read().unwrap();
        if !devs.iter().any(|d| d.device_id == id.0) {
            bail!(DomainError::DeviceNotFound(id.to_string()));
        }
        Ok(CapabilityMatrix::full_access())
    }

    fn read_file(&self, _id: &DeviceId, _path: &str) -> Result<Box<dyn std::io::Read>> {
        let content = "wireless agent file stream content".as_bytes().to_vec();
        Ok(Box::new(std::io::Cursor::new(content)))
    }

    fn push_file(&self, _id: &DeviceId, _source: &mut dyn std::io::Read, _target_path: &str) -> Result<()> {
        Ok(())
    }

    fn battery_status(&self, id: &DeviceId) -> Result<(u32, f32)> {
        let devs = self.session.devices.read().unwrap();
        let dev = devs.iter().find(|d| d.device_id == id.0);
        let bat = dev.and_then(|d| d.battery_percent).unwrap_or(90) as u32;
        let temp = dev.and_then(|d| d.temperature_c).unwrap_or(33.0);
        Ok((bat, temp))
    }

    fn list_directory(&self, _id: &DeviceId, _path: &str) -> Result<Vec<domain::FileEntry>> {
        let files = self.session.scanned_files.read().unwrap();
        Ok(files.clone())
    }

    fn delete_remote(&self, _id: &DeviceId, _path: &str) -> Result<()> {
        Ok(())
    }

    fn rename_remote(&self, _id: &DeviceId, _old_path: &str, _new_path: &str) -> Result<()> {
        Ok(())
    }

    fn copy_remote(&self, _id: &DeviceId, _source_path: &str, _target_path: &str) -> Result<()> {
        Ok(())
    }

    fn calculate_hash(&self, _id: &DeviceId, _path: &str) -> Result<String> {
        Ok("agent-sha256-hash".to_string())
    }
}

impl ScannerPort for AgentAdapter {
    fn scan(&self, device_id: &DeviceId, _roots: Vec<String>) -> Result<Vec<FileEntry>> {
        let files = self.session.scanned_files.read().unwrap();
        if !files.is_empty() {
            return Ok(files.clone());
        }

        // Return baseline remote files if not explicitly seeded
        let now = chrono::Utc::now();
        Ok(vec![
            FileEntry {
                id: domain::FileId("Pictures/agent_photo.jpg".into()),
                device_id: device_id.clone(),
                path: "Pictures/agent_photo.jpg".into(),
                name: "agent_photo.jpg".into(),
                size_bytes: 2_048_000,
                modified_at: now,
                mime_type: "image/jpeg".into(),
                permissions: "rw-".into(),
                hash_sha256: Some("agent123hash".into()),
                thumbnail_hash: None,
                media_info: None,
            },
            FileEntry {
                id: domain::FileId("Documents/agent_doc.pdf".into()),
                device_id: device_id.clone(),
                path: "Documents/agent_doc.pdf".into(),
                name: "agent_doc.pdf".into(),
                size_bytes: 512_000,
                modified_at: now,
                mime_type: "application/pdf".into(),
                permissions: "rw-".into(),
                hash_sha256: Some("agent456hash".into()),
                thumbnail_hash: None,
                media_info: None,
            },
        ])
    }
}

impl DataProviderPort for AgentAdapter {
    fn list_contacts(&self, _device_id: &DeviceId) -> Result<Vec<Contact>> {
        let data = self.session.structured_data.read().unwrap();
        if !data.contacts.is_empty() {
            return Ok(data.contacts.clone());
        }

        Ok(vec![
            Contact {
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
            }
        ])
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
    fn list_apps(&self, _device_id: &DeviceId) -> Result<Vec<App>> {
        let data = self.session.structured_data.read().unwrap();
        if !data.apps.is_empty() {
            return Ok(data.apps.clone());
        }

        Ok(vec![
            App {
                id: "com.phonebackup.agent".to_string(),
                snapshot_id: None,
                name: "Phone Backup Companion Agent".into(),
                package_name: "com.phonebackup.agent".into(),
                version_name: Some("1.0.0".into()),
                version_code: Some(1),
                is_system_app: false,
                apk_path: Some("/data/app/com.phonebackup.agent.apk".into()),
                apk_size_bytes: Some(15_000_000),
                apk_hash: None,
            }
        ])
    }

    fn export_apk(&self, _device_id: &DeviceId, _package_name: &str, _target_path: &std::path::Path) -> Result<()> {
        Ok(())
    }
}
