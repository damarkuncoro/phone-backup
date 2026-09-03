use crate::dict::models::DictionaryId;

#[cfg(feature = "derive")]
use serde::{Deserialize, Serialize};

/// Supported compression algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub enum CompressionAlgorithm {
    None,
    #[default]
    Zstd,
}

/// Compression intensity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub enum CompressionLevel {
    Fast,
    #[default]
    Balanced,
    High,
    Maximum,
    Custom(i32),
}

impl CompressionLevel {
    pub fn to_zstd_level(&self) -> i32 {
        match self {
            CompressionLevel::Fast => 1,
            CompressionLevel::Balanced => 3,
            CompressionLevel::High => 7,
            CompressionLevel::Maximum => 15,
            CompressionLevel::Custom(lvl) => *lvl,
        }
    }
}

/// High-level backup compression policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub enum CompressionPolicy {
    None,
    Fast,
    #[default]
    Balanced,
    Maximum,
    Adaptive,
}

/// Decision made by the planner on how to handle specific data chunk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompressionDecision {
    pub algorithm: CompressionAlgorithm,
    pub level: CompressionLevel,
    pub dictionary_id: Option<DictionaryId>,
    pub chunk_size: usize,
    pub enabled: bool,
    pub reason: String,
}

impl Default for CompressionDecision {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Zstd,
            level: CompressionLevel::Balanced,
            dictionary_id: None,
            chunk_size: 4 * 1024 * 1024,
            enabled: true,
            reason: "Default balanced compression".to_string(),
        }
    }
}

/// Metadata context provided to assist compression decision.
#[derive(Debug, Clone, Default)]
pub struct FileMetadataContext {
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub path: Option<String>,
    pub size: usize,
}

impl FileMetadataContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mime(mut self, mime: impl Into<String>) -> Self {
        self.mime_type = Some(mime.into());
        self
    }

    pub fn with_extension(mut self, ext: impl Into<String>) -> Self {
        self.extension = Some(ext.into());
        self
    }

    pub fn with_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
}
