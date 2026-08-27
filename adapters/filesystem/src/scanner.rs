use anyhow::Result;
use chrono::{DateTime, Utc};
use domain::{DeviceId, FileEntry, FileId};
use ports::ScannerPort;
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
}

impl ScannerPort for FilesystemScanner {
    fn scan(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        let root = Path::new(&self.root_path);

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                let metadata = entry.metadata()?;
                let relative_path = path
                    .strip_prefix(root)?
                    .to_string_lossy()
                    .into_owned();

                let modified: DateTime<Utc> = metadata.modified()?.into();
                let mime_type = mime_guess::from_path(path)
                    .first_or_octet_stream()
                    .to_string();

                entries.push(FileEntry {
                    id: FileId(relative_path.clone()),
                    device_id: device_id.clone(),
                    path: relative_path,
                    name: entry.file_name().to_string_lossy().into_owned(),
                    size_bytes: metadata.len(),
                    modified_at: modified,
                    mime_type,
                    permissions: format!("{:?}", metadata.permissions()),
                    hash_sha256: None,
                    media_info: None,
                });
            }
        }

        Ok(entries)
    }
}
