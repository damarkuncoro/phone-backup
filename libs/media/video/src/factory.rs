use crate::domain::{VideoContainer, VideoItem, VideoMetadata};
use std::path::Path;

/// Factory for rapidly creating typed `VideoItem` instances.
pub struct VideoFactory;

impl VideoFactory {
    /// Creates an MP4 video item.
    pub fn create_mp4(path: impl Into<String>, size_bytes: u64) -> VideoItem {
        VideoItem::new(path.into(), size_bytes, VideoContainer::Mp4)
    }

    /// Creates a Matroska MKV video item.
    pub fn create_mkv(path: impl Into<String>, size_bytes: u64) -> VideoItem {
        VideoItem::new(path.into(), size_bytes, VideoContainer::Mkv)
    }

    /// Creates a WebM video item.
    pub fn create_webm(path: impl Into<String>, size_bytes: u64) -> VideoItem {
        VideoItem::new(path.into(), size_bytes, VideoContainer::WebM)
    }

    /// Creates an AVI video item.
    pub fn create_avi(path: impl Into<String>, size_bytes: u64) -> VideoItem {
        VideoItem::new(path.into(), size_bytes, VideoContainer::Avi)
    }

    /// Creates a `VideoItem` by detecting container format from path extension.
    pub fn create_from_path(path: impl Into<String>, size_bytes: u64) -> VideoItem {
        let p_str = path.into();
        let ext = Path::new(&p_str)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let container = VideoContainer::from_extension(ext);
        VideoItem::new(p_str, size_bytes, container)
    }

    /// Creates a fully populated `VideoItem` with metadata.
    pub fn create_with_metadata(
        path: impl Into<String>,
        size_bytes: u64,
        metadata: VideoMetadata,
    ) -> VideoItem {
        let mut item = Self::create_from_path(path, size_bytes);
        item = item.with_metadata(metadata);
        item
    }
}
