use anyhow::Result;
use chrono::{DateTime, Utc};
use domain::{DeviceId, FileEntry, FileId};
use ports::{ScannerPort, StoragePort};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
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

                entries.push(FileEntry {
                    id: FileId(relative_path.clone()), // Placeholder ID
                    device_id: device_id.clone(),
                    path: relative_path,
                    name: entry.file_name().to_string_lossy().into_owned(),
                    size_bytes: metadata.len(),
                    modified_at: modified,
                    mime_type: "application/octet-stream".into(), // Needs real detection
                    permissions: format!("{:?}", metadata.permissions()),
                    hash_sha256: None,
                    media_info: None,
                });
            }
        }

        Ok(entries)
    }
}

pub struct LocalStorage {
    base_dir: PathBuf,
}

impl LocalStorage {
    pub fn new(base_dir: impl Into<PathBuf>) -> Result<Self> {
        let base_dir = base_dir.into();
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }
        Ok(Self { base_dir })
    }
}

impl StoragePort for LocalStorage {
    fn write(&self, id: &str, data: &mut dyn Read) -> Result<()> {
        let path = self.base_dir.join(id);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        io::copy(data, &mut file)?;
        Ok(())
    }

    fn read(&self, id: &str) -> Result<Box<dyn Read>> {
        let path = self.base_dir.join(id);
        let file = File::open(path)?;
        Ok(Box::new(file))
    }

    fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.base_dir.join(id).exists())
    }

    fn delete(&self, id: &str) -> Result<()> {
        let path = self.base_dir.join(id);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
