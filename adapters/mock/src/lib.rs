//! `MockDeviceAdapter`: a fake `DevicePort` implementation.
//!
//! Purpose: prove that `application::BackupService` never needs to
//! know or care which adapter it's talking to. When Phase 02 lands
//! `AdapterAdb::AdbDeviceAdapter`, it drops in here with zero changes
//! to `application` or `cli` beyond which adapter gets constructed.

use anyhow::{bail, Result};
use domain::{
    Capability, CapabilityMatrix, CapabilityStatus, ConnectionType, Device, DeviceId,
    FileEntry, FileId, AppInfo, AppId, Contact, Sms, CallLog,
};
use ports::{DevicePort, ScannerPort, AppProviderPort, DataProviderPort};
use chrono::Utc;

pub struct MockDeviceAdapter {
    devices: Vec<Device>,
}

pub struct MockAppProvider;

impl AppProviderPort for MockAppProvider {
    fn list_apps(&self, device_id: &DeviceId) -> Result<Vec<AppInfo>> {
        Ok(vec![
            AppInfo {
                id: AppId("com.whatsapp".into()),
                device_id: device_id.clone(),
                package_name: "com.whatsapp".into(),
                version_name: "2.24.1".into(),
                version_code: 2401,
                installer: Some("com.android.vending".into()),
                app_name: "WhatsApp".into(),
            },
            AppInfo {
                id: AppId("com.instagram.android".into()),
                device_id: device_id.clone(),
                package_name: "com.instagram.android".into(),
                version_name: "315.0.0".into(),
                version_code: 31500,
                installer: Some("com.android.vending".into()),
                app_name: "Instagram".into(),
            },
        ])
    }

    fn get_apk(&self, _device_id: &DeviceId, _package_name: &str) -> Result<Box<dyn std::io::Read>> {
        let content = vec![0u8; 1024]; // Dummy APK content
        Ok(Box::new(std::io::Cursor::new(content)))
    }

    fn install_app(&self, _device_id: &DeviceId, _apk_data: &mut dyn std::io::Read) -> Result<()> {
        Ok(())
    }
}

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
        Ok(vec![
            Sms {
                address: "+123456789".into(),
                body: "Hello, this is a test SMS".into(),
                date: Utc::now(),
                type_code: 1,
            },
        ])
    }

    fn list_call_logs(&self, _device_id: &DeviceId) -> Result<Vec<CallLog>> {
        Ok(vec![
            CallLog {
                number: "+123456789".into(),
                date: Utc::now(),
                duration_seconds: 120,
                type_code: 1,
            },
        ])
    }
}

#[derive(Default)]
pub struct MockScannerAdapter;

impl ScannerPort for MockScannerAdapter {
    fn scan(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        let stable_time = chrono::DateTime::parse_from_rfc3339("2026-08-27T00:00:00Z").unwrap().with_timezone(&Utc);
        Ok(vec![
            FileEntry {
                id: FileId("DCIM/Camera/IMG_001.jpg".into()),
                device_id: device_id.clone(),
                path: "DCIM/Camera/IMG_001.jpg".into(),
                name: "IMG_001.jpg".into(),
                size_bytes: 4_283_921,
                modified_at: stable_time,
                mime_type: "image/jpeg".into(),
                permissions: "rw-".into(),
                hash_sha256: Some("abc123hash".into()),
                media_info: None,
            },
            FileEntry {
                id: FileId("Documents/resume.pdf".into()),
                device_id: device_id.clone(),
                path: "Documents/resume.pdf".into(),
                name: "resume.pdf".into(),
                size_bytes: 1_048_576,
                modified_at: stable_time,
                mime_type: "application/pdf".into(),
                permissions: "rw-".into(),
                hash_sha256: Some("def456hash".into()),
                media_info: None,
            },
            FileEntry {
                id: FileId("Documents/notes.txt".into()),
                device_id: device_id.clone(),
                path: "Documents/notes.txt".into(),
                name: "notes.txt".into(),
                size_bytes: 512,
                modified_at: stable_time,
                mime_type: "text/plain".into(),
                permissions: "rw-".into(),
                hash_sha256: None,
                media_info: None,
            },
        ])
    }
}

impl Default for MockDeviceAdapter {
    fn default() -> Self {
        Self {
            devices: vec![Device {
                id: DeviceId::new("A1B2C3D4"),
                manufacturer: "Google".into(),
                model: "Pixel 8".into(),
                serial: "A1B2C3D4".into(),
                os_version: "Android 15".into(),
                sdk_version: Some(35),
                storage_total_bytes: 256_000_000_000,
                storage_used_bytes: 184_000_000_000,
                storage_free_bytes: 72_000_000_000,
                connection_type: ConnectionType::Usb,
            }],
        }
    }
}

impl DevicePort for MockDeviceAdapter {
    fn discover(&self) -> Result<Vec<Device>> {
        Ok(self.devices.clone())
    }

    fn info(&self, id: &DeviceId) -> Result<Device> {
        self.devices
            .iter()
            .find(|d| &d.id == id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!(domain::DomainError::DeviceNotFound(id.to_string())))
    }

    fn capabilities(&self, id: &DeviceId) -> Result<CapabilityMatrix> {
        if self.devices.iter().all(|d| &d.id != id) {
            bail!(domain::DomainError::DeviceNotFound(id.to_string()));
        }
        let mut matrix = CapabilityMatrix::new();
        matrix.set(Capability::ReadFiles, CapabilityStatus::Available);
        matrix.set(Capability::ReadMedia, CapabilityStatus::Available);
        matrix.set(Capability::ReadDownload, CapabilityStatus::Available);
        matrix.set(Capability::ReadDocuments, CapabilityStatus::Available);
        matrix.set(Capability::ReadAppData, CapabilityStatus::RequiresUserAction);
        matrix.set(Capability::ReadContacts, CapabilityStatus::RequiresUserAction);
        matrix.set(Capability::ReadSms, CapabilityStatus::Denied);
        matrix.set(Capability::ReadCallLog, CapabilityStatus::Denied);
        Ok(matrix)
    }

    fn read_file(&self, _id: &DeviceId, _path: &str) -> Result<Box<dyn std::io::Read>> {
        // Return some dummy content for mock
        let content = "this is mock file content".as_bytes().to_vec();
        Ok(Box::new(std::io::Cursor::new(content)))
    }

    fn push_file(&self, _id: &DeviceId, _source: &mut dyn std::io::Read, _target_path: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_the_seeded_device() {
        let adapter = MockDeviceAdapter::default();
        let devices = adapter.discover().unwrap();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].model, "Pixel 8");
    }

    #[test]
    fn unknown_device_info_errors() {
        let adapter = MockDeviceAdapter::default();
        let err = adapter.info(&DeviceId::new("NOPE"));
        assert!(err.is_err());
    }

    #[test]
    fn capability_matrix_flags_protected_data_as_denied() {
        let adapter = MockDeviceAdapter::default();
        let id = DeviceId::new("A1B2C3D4");
        let matrix = adapter.capabilities(&id).unwrap();
        assert!(matrix.is_available(Capability::ReadFiles));
        assert!(!matrix.is_available(Capability::ReadSms));
    }
}
