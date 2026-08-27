use anyhow::Result;
use chrono::Utc;
use domain::{DeviceId, FileEntry, FileId};
use ports::ScannerPort;

#[derive(Default)]
pub struct MockScannerAdapter;

impl ScannerPort for MockScannerAdapter {
    fn scan(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        let stable_time = chrono::DateTime::parse_from_rfc3339("2026-08-27T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
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
