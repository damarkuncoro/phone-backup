use crate::FileEntry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// High-level file categorization inferred during scanning.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ScanCategory {
    Photos,
    Videos,
    Audio,
    Documents,
    WhatsApp,
    Apks,
    Downloads,
    System,
    Other,
}

impl std::fmt::Display for ScanCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Photos => write!(f, "Photos"),
            Self::Videos => write!(f, "Videos"),
            Self::Audio => write!(f, "Audio"),
            Self::Documents => write!(f, "Documents"),
            Self::WhatsApp => write!(f, "WhatsApp"),
            Self::Apks => write!(f, "APKs"),
            Self::Downloads => write!(f, "Downloads"),
            Self::System => write!(f, "System"),
            Self::Other => write!(f, "Other"),
        }
    }
}

/// Aggregated metrics per category.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ScanCategorySummary {
    pub file_count: usize,
    pub total_bytes: u64,
}

/// Real-time scanner performance and throughput metrics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScanMetrics {
    pub duration_ms: u64,
    pub directories_scanned: usize,
    pub files_scanned: usize,
    pub throughput_files_per_sec: f64,
}

/// Advanced rule-based filter applied during discovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanFilter {
    pub exclude_noise: bool,
    pub exclude_thumbnails: bool,
    pub exclude_cache: bool,
    pub exclude_trash: bool,
    pub exclude_nomedia: bool,
    pub min_size_bytes: Option<u64>,
    pub max_size_bytes: Option<u64>,
    pub custom_exclude_globs: Vec<String>,
}

impl Default for ScanFilter {
    fn default() -> Self {
        Self {
            exclude_noise: true,
            exclude_thumbnails: true,
            exclude_cache: true,
            exclude_trash: true,
            exclude_nomedia: true,
            min_size_bytes: None,
            max_size_bytes: None,
            custom_exclude_globs: Vec::new(),
        }
    }
}

/// Enumeration of scan data sources.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScanSource {
    MediaStoreImages,
    MediaStoreVideos,
    MediaStoreAudio,
    FileSystem,
    WhatsAppMedia,
    AppleAfc,
    AgentCompanion,
}

/// Structured warning for non-fatal scan anomalies (e.g. Scoped Storage restrictions).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanWarning {
    pub source: ScanSource,
    pub path: String,
    pub message: String,
}

/// Consolidated scan result containing discovered files, categories, metrics, and warnings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScanResult {
    pub files: Vec<FileEntry>,
    pub warnings: Vec<ScanWarning>,
    pub categories: BTreeMap<ScanCategory, ScanCategorySummary>,
    pub metrics: Option<ScanMetrics>,
}

impl ScanResult {
    pub fn new(files: Vec<FileEntry>, warnings: Vec<ScanWarning>) -> Self {
        Self {
            files,
            warnings,
            categories: BTreeMap::new(),
            metrics: None,
        }
    }

    pub fn with_details(
        files: Vec<FileEntry>,
        warnings: Vec<ScanWarning>,
        categories: BTreeMap<ScanCategory, ScanCategorySummary>,
        metrics: Option<ScanMetrics>,
    ) -> Self {
        Self {
            files,
            warnings,
            categories,
            metrics,
        }
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

    pub fn total_bytes(&self) -> u64 {
        self.files.iter().map(|f| f.size_bytes).sum()
    }
}
