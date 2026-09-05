use super::AgentAdapter;
use anyhow::Result;
use domain::{DeviceId, FileEntry};
use ports::ScannerPort;

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

    fn scan_detailed(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
        filter: Option<&domain::ScanFilter>,
    ) -> Result<domain::ScanResult> {
        let files = self.scan(device_id, roots.clone())?;
        Ok(scanner_engine::ScanPipeline::process_source(files, roots.len(), filter, Vec::new()))
    }
}
