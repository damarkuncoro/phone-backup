use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{self, Read};
use domain::{FileEntry, FileId, DeviceId};

pub struct MtpFileOperations {
    root_path: PathBuf,
}

impl MtpFileOperations {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    pub fn resolve_path(&self, rel_path: &str) -> PathBuf {
        let clean_rel = rel_path.trim_start_matches('/').trim_start_matches("sdcard/");
        if clean_rel.is_empty() {
            self.root_path.clone()
        } else {
            self.root_path.join(clean_rel)
        }
    }

    pub fn read_file(&self, path: &str) -> Result<Box<dyn Read>> {
        let local_path = self.resolve_path(path);
        let file = File::open(&local_path)
            .map_err(|e| anyhow!("Failed to read MTP file {:?}: {}", local_path, e))?;
        Ok(Box::new(file))
    }

    pub fn push_file(&self, source: &mut dyn Read, target_path: &str) -> Result<()> {
        let dest = self.resolve_path(target_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&dest)?;
        io::copy(source, &mut file)?;
        Ok(())
    }

    pub fn list_directory(&self, device_id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        let dir_path = self.resolve_path(path);
        if !dir_path.exists() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        for entry in fs::read_dir(&dir_path)?.flatten() {
            let p = entry.path();
            let is_dir = p.is_dir();
            let metadata = entry.metadata()?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            let virtual_path = format!(
                "{}/{}",
                path.trim_end_matches('/'),
                file_name
            );

            entries.push(FileEntry {
                id: FileId(virtual_path.clone()),
                device_id: device_id.clone(),
                path: virtual_path,
                name: file_name,
                size_bytes: if is_dir { 0 } else { metadata.len() },
                modified_at: chrono::Utc::now(),
                mime_type: if is_dir {
                    "inode/directory".to_string()
                } else {
                    "application/octet-stream".to_string()
                },
                permissions: if is_dir { "drwxr-xr-x".to_string() } else { "-rw-r--r--".to_string() },
                hash_sha256: None,
                thumbnail_hash: None,
                media_info: None,
            });
        }
        Ok(entries)
    }

    pub fn delete(&self, path: &str) -> Result<()> {
        let p = self.resolve_path(path);
        if p.is_dir() {
            fs::remove_dir_all(p)?;
        } else if p.exists() {
            fs::remove_file(p)?;
        }
        Ok(())
    }

    pub fn rename(&self, old_path: &str, new_path: &str) -> Result<()> {
        let src = self.resolve_path(old_path);
        let dest = self.resolve_path(new_path);
        fs::rename(src, dest)?;
        Ok(())
    }

    pub fn copy(&self, source_path: &str, target_path: &str) -> Result<()> {
        let src = self.resolve_path(source_path);
        let dest = self.resolve_path(target_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dest)?;
        Ok(())
    }
}
