use crate::domain::{VideoContainer, VideoItem, VideoMetadata};
use chrono::{DateTime, Utc};

/// Fluent builder for constructing `VideoItem` aggregates.
#[derive(Default)]
pub struct VideoItemBuilder {
    path: Option<String>,
    size_bytes: Option<u64>,
    container: Option<VideoContainer>,
    modified_at: Option<DateTime<Utc>>,
    metadata: Option<VideoMetadata>,
}

impl VideoItemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn size_bytes(mut self, size: u64) -> Self {
        self.size_bytes = Some(size);
        self
    }

    pub fn container(mut self, container: VideoContainer) -> Self {
        self.container = Some(container);
        self
    }

    pub fn modified_at(mut self, modified: DateTime<Utc>) -> Self {
        self.modified_at = Some(modified);
        self
    }

    pub fn metadata(mut self, metadata: VideoMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Builds the `VideoItem`, inferring container from path if not explicitly provided.
    pub fn build(self) -> Result<VideoItem, &'static str> {
        let path = self.path.ok_or("Video path is required")?;
        let size_bytes = self.size_bytes.unwrap_or(0);
        let container = self.container.unwrap_or_else(|| {
            let ext = std::path::Path::new(&path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            VideoContainer::from_extension(ext)
        });

        let mut item = VideoItem::new(path, size_bytes, container);
        if let Some(modified) = self.modified_at {
            item = item.with_modified(modified);
        }
        if let Some(meta) = self.metadata {
            item = item.with_metadata(meta);
        }

        Ok(item)
    }
}
