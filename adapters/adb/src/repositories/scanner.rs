use crate::client::AdbClient;
use crate::parsers::media_parser::MediaParser;
use crate::scripts::AndroidScripts;
use anyhow::{Context, Result};
use domain::{DeviceId, FileEntry};
use std::collections::BTreeMap;

const DEFAULT_SCAN_ROOTS: &[&str] = &[
    "/storage/emulated/0/DCIM",
    "/storage/emulated/0/Pictures",
    "/storage/emulated/0/Movies",
    "/storage/emulated/0/Download",
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

    fn scan_mediastore(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        let mut all_media = Vec::new();

        let image_script = AndroidScripts::query_mediastore("image");
        if let Ok(image_out) = self.client.shell(&device_id.0, &image_script) {
            all_media.extend(MediaParser::parse_mediastore(device_id, &image_out));
        }

        let video_script = AndroidScripts::query_mediastore("video");
        if let Ok(video_out) = self.client.shell(&device_id.0, &video_script) {
            all_media.extend(MediaParser::parse_mediastore(device_id, &video_out));
        }

        Ok(all_media)
    }

    fn scan_filesystem(&self, device_id: &DeviceId, roots: &[String]) -> Result<Vec<FileEntry>> {
        let script = AndroidScripts::find_files(roots);
        let stdout = self
            .client
            .shell(&device_id.0, &script)
            .context("Failed to execute Android filesystem scan")?;

        Ok(MediaParser::parse_filesystem_scan(device_id, &stdout))
    }

    fn merge_file_entries(mediastore: FileEntry, filesystem: FileEntry) -> FileEntry {
        FileEntry {
            id: mediastore.id,
            device_id: filesystem.device_id,
            path: mediastore.path,
            name: filesystem.name,
            size_bytes: filesystem.size_bytes,
            modified_at: filesystem.modified_at,
            mime_type: if mediastore.mime_type.is_empty() {
                filesystem.mime_type
            } else {
                mediastore.mime_type
            },
            permissions: filesystem.permissions,
            hash_sha256: filesystem.hash_sha256.or(mediastore.hash_sha256),
            thumbnail_hash: mediastore.thumbnail_hash.or(filesystem.thumbnail_hash),
            media_info: mediastore.media_info.or(filesystem.media_info),
        }
    }

    pub fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        let media_entries = self.scan_mediastore(device_id)?;
        let scan_roots = self.resolve_roots(roots);
        let filesystem_entries = self.scan_filesystem(device_id, &scan_roots)?;

        let mut entries_map = BTreeMap::<String, FileEntry>::new();

        for media in media_entries {
            entries_map.insert(media.path.clone(), media);
        }

        for fs_file in filesystem_entries {
            if let Some(existing_media) = entries_map.remove(&fs_file.path) {
                let merged = Self::merge_file_entries(existing_media, fs_file);
                entries_map.insert(merged.path.clone(), merged);
            } else {
                entries_map.insert(fs_file.path.clone(), fs_file);
            }
        }

        Ok(entries_map.into_values().collect())
    }
}
