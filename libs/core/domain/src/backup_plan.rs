use crate::FileEntry;
use serde::{Deserialize, Serialize};

/// Represents an unchanged file from a previous snapshot that is reused without re-uploading.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileReuse {
    pub path: String,
    pub size_bytes: u64,
    pub hash_sha256: Option<String>,
}

/// Represents a file that was present in a previous snapshot but is missing from the current device scan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeletedFile {
    pub path: String,
    pub size_bytes: u64,
}

/// Comprehensive plan detailing upload, reuse, skipped, and deleted file classifications.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BackupPlan {
    pub upload: Vec<FileEntry>,
    pub reuse: Vec<FileReuse>,
    pub skipped: Vec<FileEntry>,
    pub deleted: Vec<DeletedFile>,
    pub logical_bytes: u64,
    pub upload_bytes: u64,
}

impl BackupPlan {
    pub fn new(
        upload: Vec<FileEntry>,
        reuse: Vec<FileReuse>,
        skipped: Vec<FileEntry>,
        deleted: Vec<DeletedFile>,
    ) -> Self {
        let upload_bytes: u64 = upload.iter().map(|f| f.size_bytes).sum();
        let reuse_bytes: u64 = reuse.iter().map(|f| f.size_bytes).sum();
        let logical_bytes = upload_bytes + reuse_bytes;

        Self {
            upload,
            reuse,
            skipped,
            deleted,
            logical_bytes,
            upload_bytes,
        }
    }

    pub fn upload_count(&self) -> usize {
        self.upload.len()
    }

    pub fn reuse_count(&self) -> usize {
        self.reuse.len()
    }

    pub fn deleted_count(&self) -> usize {
        self.deleted.len()
    }
}
