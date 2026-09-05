use anyhow::Result;
use chrono::{DateTime, Utc};
use domain::{DeviceId, FileEntry, FileId, ScanFilter, ScanResult};
use ports::ScannerPort;
use scanner_engine::ScanPipeline;
use std::path::Path;
use walkdir::WalkDir;

pub struct FilesystemScanner {
    root_path: String,
}

impl FilesystemScanner {
    pub fn new(root_path: impl Into<String>) -> Self {
        Self {
            root_path: root_path.into(),
        }
    }
    fn entry_to_file(
        entry: &walkdir::DirEntry,
        base_root: &Path,
        device_id: DeviceId,
    ) -> Result<FileEntry> {
        let path = entry.path();
        let metadata = entry.metadata()?;
        let relative_path = path.strip_prefix(base_root)?.to_string_lossy().into_owned();

        let modified: DateTime<Utc> = metadata.modified()?.into();
        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        Ok(FileEntry {
            id: FileId(relative_path.clone()),
            device_id,
            path: relative_path,
            name: entry.file_name().to_string_lossy().into_owned(),
            size_bytes: metadata.len(),
            modified_at: modified,
            mime_type,
            permissions: format!("{:?}", metadata.permissions()),
            hash_sha256: None,
            thumbnail_hash: None,
            media_info: None,
        })
    }
}

impl ScannerPort for FilesystemScanner {
    fn scan(&self, device_id: &DeviceId, roots: Vec<String>) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        let base_root = Path::new(&self.root_path);

        let scan_roots: Vec<String> = if roots.is_empty() {
            vec![".".to_string()]
        } else {
            roots
        };

        for root_suffix in scan_roots {
            let root = base_root.join(root_suffix);
            if !root.exists() {
                continue;
            }

            for entry in WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    if let Ok(file_entry) =
                        Self::entry_to_file(&entry, base_root, device_id.clone())
                    {
                        entries.push(file_entry);
                    }
                }
            }
        }

        Ok(entries)
    }

    fn scan_detailed(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
        filter: Option<&ScanFilter>,
    ) -> Result<ScanResult> {
        let files = self.scan(device_id, roots.clone())?;
        Ok(ScanPipeline::process_source(files, roots.len(), filter, Vec::new()))
    }
}
