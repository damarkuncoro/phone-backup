use crate::client::AdbClient;
use crate::parsers::media_parser::MediaParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::{DeviceId, FileEntry};
use std::collections::HashMap;

const DEFAULT_SCAN_ROOTS: &[&str] = &[
    "/storage/emulated/0/DCIM",
    "/storage/emulated/0/Pictures",
    "/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media",
    "/storage/emulated/0/WhatsApp/Media",
];

#[derive(Clone)]
pub struct AdbScannerRepository {
    client: AdbClient,
}

impl AdbScannerRepository {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    fn resolve_roots(&self, provided_roots: Vec<String>) -> Vec<String> {
        if provided_roots.is_empty() {
            DEFAULT_SCAN_ROOTS.iter().map(|s| s.to_string()).collect()
        } else {
            provided_roots
        }
    }

    fn scan_mediastore(&self, device_id: &DeviceId) -> Vec<FileEntry> {
        let mut all_media = Vec::new();
        if let Ok(out) = self.client.shell(&device_id.0, &AndroidScripts::query_mediastore("image")) {
            all_media.extend(MediaParser::parse_mediastore(device_id, &out));
        }
        if let Ok(out) = self.client.shell(&device_id.0, &AndroidScripts::query_mediastore("video")) {
            all_media.extend(MediaParser::parse_mediastore(device_id, &out));
        }
        all_media
    }

    pub fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        let media_entries = self.scan_mediastore(device_id);
        let mut entries_map: HashMap<String, FileEntry> = media_entries.into_iter()
            .map(|f| (f.path.clone(), f))
            .collect();

        let scan_roots = self.resolve_roots(roots);
        let script = AndroidScripts::find_files(&scan_roots);

        if let Ok(stdout) = self.client.shell(&device_id.0, &script) {
            let fs_entries = MediaParser::parse_filesystem_scan(device_id, &stdout);
            for fs_file in fs_entries {
                entries_map.entry(fs_file.path.clone()).or_insert(fs_file);
            }
        }

        Ok(entries_map.into_values().collect())
    }
}
