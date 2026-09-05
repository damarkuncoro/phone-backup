use crate::operations::MtpFileOperations;
use anyhow::Result;
use domain::{DeviceId, FileEntry, FileId, ScanFilter, ScanResult};
use ports::ScannerPort;
use scanner_engine::ScanPipeline;
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
    fn resolve_virtual_path(base: &str, rel: &std::path::Path) -> String {
        if base.is_empty() || base == "/" {
            format!("/{}", rel.to_string_lossy().trim_start_matches('/'))
        } else {
            format!(
                "/{}/{}",
                base.trim_matches('/'),
                rel.to_string_lossy().trim_start_matches('/')
            )
        }
    }

    fn entry_to_file(
        entry: &walkdir::DirEntry,
        virtual_path: String,
        device_id: DeviceId,
    ) -> Result<FileEntry> {
        let meta = entry.metadata()?;
        let mime_type = mime_guess::from_path(entry.path())
            .first_or_octet_stream()
            .to_string();

        let modified: chrono::DateTime<chrono::Utc> =
            meta.modified().map(Into::into).unwrap_or_else(|_| chrono::Utc::now());

        Ok(FileEntry {
            id: FileId(virtual_path.clone()),
            device_id,
            path: virtual_path,
            name: entry.file_name().to_string_lossy().to_string(),
            size_bytes: meta.len(),
            modified_at: modified,
            mime_type,
            permissions: "-rw-r--r--".to_string(),
            hash_sha256: None,
            thumbnail_hash: None,
            media_info: None,
        })
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
                if entry.file_type().is_dir() {
                    continue;
                }

                let p = entry.path();
                let rel = p.strip_prefix(&base_dir).unwrap_or(p);
                let virtual_path = Self::resolve_virtual_path(&base, rel);

                if let Ok(file_entry) = Self::entry_to_file(&entry, virtual_path, id.clone()) {
                    results.push(file_entry);
                }
            }
        }
        Ok(results)
    }

    fn scan_detailed(
        &self,
        id: &DeviceId,
        target_paths: Vec<String>,
        filter: Option<&ScanFilter>,
    ) -> Result<ScanResult> {
        let files = self.scan(id, target_paths.clone())?;
        Ok(ScanPipeline::process_source(files, target_paths.len(), filter, Vec::new()))
    }
}
