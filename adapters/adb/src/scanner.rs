use crate::client::AdbClient;
use anyhow::Result;
use chrono::{TimeZone, Utc};
use domain::{DeviceId, FileEntry, FileId};
use ports::ScannerPort;

pub struct AdbScannerAdapter {
    client: AdbClient,
}

impl AdbScannerAdapter {
    pub fn new() -> Self {
        Self {
            client: AdbClient::new(),
        }
    }
}

impl Default for AdbScannerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ScannerPort for AdbScannerAdapter {
    fn scan(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        let script = "find /sdcard/ -type f -maxdepth 4 2>/dev/null | xargs stat -c '%n|%s|%Y' 2>/dev/null";

        let stdout = self.client.shell(&device_id.0, script)?;
        let mut entries = Vec::new();

        for line in stdout.lines() {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 3 {
                continue;
            }

            let path = parts[0].to_string();
            let size_bytes = parts[1].parse::<u64>().unwrap_or(0);
            let mtime_unix = parts[2].parse::<i64>().unwrap_or(0);

            let modified_at = Utc.timestamp_opt(mtime_unix, 0).single().unwrap_or_else(Utc::now);
            let mime_type = mime_guess::from_path(&path).first_or_octet_stream().to_string();

            entries.push(FileEntry {
                id: FileId(path.clone()),
                device_id: device_id.clone(),
                path: path.clone(),
                name: path.split('/').last().unwrap_or("").to_string(),
                size_bytes,
                modified_at,
                mime_type,
                permissions: "".into(),
                hash_sha256: None,
                media_info: None,
            });
        }

        Ok(entries)
    }
}
