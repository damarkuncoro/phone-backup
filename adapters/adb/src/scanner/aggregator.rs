use super::filesystem_scanner::FileSystemScanner;
use super::mediastore_scanner::MediaStoreScanner;
use crate::client::AdbClient;
use anyhow::Result;
use domain::{DeviceId, FileEntry};
use std::collections::BTreeMap;

pub const DEFAULT_SCAN_ROOTS: &[&str] = &[
    "/storage/emulated/0/DCIM",
    "/storage/emulated/0/Pictures",
    "/storage/emulated/0/Movies",
    "/storage/emulated/0/Download",
    "/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media",
    "/storage/emulated/0/WhatsApp/Media",
];

/// Coordinator aggregating MediaStore and FileSystem sub-scanners with deterministic sorting.
#[derive(Clone)]
pub struct ScannerAggregator {
    mediastore_scanner: MediaStoreScanner,
    filesystem_scanner: FileSystemScanner,
}

impl ScannerAggregator {
    pub fn new(client: AdbClient) -> Self {
        Self {
            mediastore_scanner: MediaStoreScanner::new(client.clone()),
            filesystem_scanner: FileSystemScanner::new(client),
        }
    }

    fn resolve_roots(&self, provided_roots: Vec<String>) -> Vec<String> {
        if provided_roots.is_empty() {
            DEFAULT_SCAN_ROOTS.iter().map(|s| s.to_string()).collect()
        } else {
            provided_roots
        }
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
        let res = self.scan_with_result(device_id, roots)?;
        Ok(res.files)
    }

    pub fn scan_with_result(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<domain::ScanResult> {
        let mut warnings = Vec::new();

        let media_entries = match self.mediastore_scanner.scan(device_id) {
            Ok(entries) => entries,
            Err(e) => {
                warnings.push(domain::ScanWarning {
                    source: domain::ScanSource::MediaStoreImages,
                    path: "MediaStore".to_string(),
                    message: format!("MediaStore query warning: {}", e),
                });
                Vec::new()
            }
        };

        let scan_roots = self.resolve_roots(roots);
        let filesystem_entries = match self.filesystem_scanner.scan(device_id, &scan_roots) {
            Ok(entries) => entries,
            Err(e) => {
                warnings.push(domain::ScanWarning {
                    source: domain::ScanSource::FileSystem,
                    path: scan_roots.join(", "),
                    message: format!("Filesystem scan warning: {}", e),
                });
                Vec::new()
            }
        };

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

        Ok(domain::ScanResult::new(entries_map.into_values().collect(), warnings))
    }
}
