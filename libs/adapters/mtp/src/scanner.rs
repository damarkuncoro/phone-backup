use crate::operations::MtpFileOperations;
use anyhow::Result;
use domain::{DeviceId, FileEntry, FileId};
use ports::ScannerPort;
use std::path::PathBuf;
use tracing::info;

pub struct MtpScanner {
    ops: MtpFileOperations,
}

impl MtpScanner {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            ops: MtpFileOperations::new(root_path),
        }
    }
}

impl ScannerPort for MtpScanner {
    fn scan(&self, id: &DeviceId, target_paths: Vec<String>) -> Result<Vec<FileEntry>> {
        info!("Scanning MTP device {}", id);
        let paths_to_scan = if target_paths.is_empty() {
            vec!["".to_string()]
        } else {
            target_paths
        };

        let mut results = Vec::new();
        for base in paths_to_scan {
            let base_dir = self.ops.resolve_path(&base);
            if !base_dir.exists() {
                continue;
            }

            for entry in walkdir::WalkDir::new(&base_dir).into_iter().flatten() {
                let p = entry.path();
                if p.is_dir() {
                    continue;
                }

                let meta = entry.metadata()?;
                let rel = p.strip_prefix(&base_dir).unwrap_or(p);

                let virtual_path = if base.is_empty() || base == "/" {
                    format!("/{}", rel.to_string_lossy().trim_start_matches('/'))
                } else {
                    format!(
                        "/{}/{}",
                        base.trim_matches('/'),
                        rel.to_string_lossy().trim_start_matches('/')
                    )
                };

                results.push(FileEntry {
                    id: FileId(virtual_path.clone()),
                    device_id: id.clone(),
                    path: virtual_path,
                    name: entry.file_name().to_string_lossy().to_string(),
                    size_bytes: meta.len(),
                    modified_at: chrono::Utc::now(),
                    mime_type: "application/octet-stream".to_string(),
                    permissions: "-rw-r--r--".to_string(),
                    hash_sha256: None,
                    thumbnail_hash: None,
                    media_info: None,
                });
            }
        }
        Ok(results)
    }
}
