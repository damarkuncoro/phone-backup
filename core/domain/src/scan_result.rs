use crate::FileEntry;
use serde::{Deserialize, Serialize};

/// Enumeration of scan data sources.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScanSource {
    MediaStoreImages,
    MediaStoreVideos,
    FileSystem,
    WhatsAppMedia,
}

/// Structured warning for non-fatal scan anomalies (e.g. Scoped Storage restrictions).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanWarning {
    pub source: ScanSource,
    pub path: String,
    pub message: String,
}

/// Consolidated scan result containing discovered files and non-fatal warnings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScanResult {
    pub files: Vec<FileEntry>,
    pub warnings: Vec<ScanWarning>,
}

impl ScanResult {
    pub fn new(files: Vec<FileEntry>, warnings: Vec<ScanWarning>) -> Self {
        Self { files, warnings }
    }

    pub fn is_successful(&self) -> bool {
        self.warnings.is_empty()
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}
