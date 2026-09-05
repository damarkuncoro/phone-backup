use super::video_metadata::VideoMetadata;
use super::video_type::{VideoContainer, VideoQuality};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Aggregate root representing a single video file and its metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoItem {
    pub path: String,
    pub size_bytes: u64,
    pub modified_at: Option<DateTime<Utc>>,
    pub container: VideoContainer,
    pub metadata: Option<VideoMetadata>,
}

impl VideoItem {
    /// Creates a new basic VideoItem without metadata.
    pub fn new(path: String, size_bytes: u64, container: VideoContainer) -> Self {
        Self {
            path,
            size_bytes,
            modified_at: None,
            container,
            metadata: None,
        }
    }

    /// Sets the technical metadata for this video.
    pub fn with_metadata(mut self, metadata: VideoMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Sets modification timestamp.
    pub fn with_modified(mut self, modified: DateTime<Utc>) -> Self {
        self.modified_at = Some(modified);
        self
    }

    /// Returns quality tier (or Unknown if metadata not extracted).
    pub fn quality(&self) -> VideoQuality {
        self.metadata
            .as_ref()
            .map(|m| m.quality_tier)
            .unwrap_or(VideoQuality::Unknown)
    }

    /// Returns duration in seconds or 0.0 if not available.
    pub fn duration_secs(&self) -> f64 {
        self.metadata.as_ref().map(|m| m.duration_secs).unwrap_or(0.0)
    }

    /// Returns formatted resolution string.
    pub fn resolution(&self) -> String {
        self.metadata
            .as_ref()
            .map(|m| m.format_resolution())
            .unwrap_or_else(|| "-".to_string())
    }

    /// Returns formatted duration string.
    pub fn duration_display(&self) -> String {
        self.metadata
            .as_ref()
            .map(|m| m.format_duration())
            .unwrap_or_else(|| "--:--".to_string())
    }
}
