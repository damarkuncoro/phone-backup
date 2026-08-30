use crate::client::AdbClient;
use crate::parsers::media_parser::MediaParser;
use crate::scripts::AndroidScripts;
use anyhow::Result;
use domain::{DeviceId, FileEntry};
use ports::ScannerPort;
use std::collections::HashMap;

const DEFAULT_SCAN_ROOTS: &[&str] = &[
    "/storage/emulated/0/DCIM",
    "/storage/emulated/0/Pictures",
    "/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media",
    "/storage/emulated/0/WhatsApp/Media",
];

pub struct AdbScannerGateway {
    client: AdbClient,
}

impl AdbScannerGateway {
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

        // Query Images
        if let Ok(out) = self.client.shell(&device_id.0, &AndroidScripts::query_mediastore("image")) {
            all_media.extend(MediaParser::parse_mediastore(device_id, &out));
        }

        // Query Videos
        if let Ok(out) = self.client.shell(&device_id.0, &AndroidScripts::query_mediastore("video")) {
            all_media.extend(MediaParser::parse_mediastore(device_id, &out));
        }

        all_media
    }
}

impl ScannerPort for AdbScannerGateway {
    fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        // 1. Perform rich MediaStore scan first
        let media_entries = self.scan_mediastore(device_id);
        let mut entries_map: HashMap<String, FileEntry> = media_entries.into_iter()
            .map(|f| (f.path.clone(), f))
            .collect();

        // 2. Perform deep filesystem scan to catch non-media files or files outside MediaStore
        let scan_roots = self.resolve_roots(roots);
        let script = AndroidScripts::find_files(&scan_roots);

        if let Ok(stdout) = self.client.shell(&device_id.0, &script) {
            let fs_entries = MediaParser::parse_filesystem_scan(device_id, &stdout);

            for fs_file in fs_entries {
                // If we already have this file from MediaStore, keep the one with rich metadata.
                // Otherwise, add it.
                entries_map.entry(fs_file.path.clone()).or_insert(fs_file);
            }
        }

        Ok(entries_map.into_values().collect())
    }
}
