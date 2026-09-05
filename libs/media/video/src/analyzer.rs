use crate::domain::{VideoContainer, VideoItem, VideoMetadata};
use crate::extractors::{MkvExtractor, Mp4Extractor, RiffExtractor};

/// Domain service for analyzing video files, extracting metadata and categorizing.
pub struct VideoAnalyzer;

impl VideoAnalyzer {
    /// Analyzes raw byte prefix/slice of a video file and produces metadata.
    pub fn analyze_bytes(container: &VideoContainer, data: &[u8]) -> Option<VideoMetadata> {
        match container {
            VideoContainer::Mp4 | VideoContainer::Mov | VideoContainer::ThreeGP => {
                Mp4Extractor::extract_from_bytes(data)
            }
            VideoContainer::Mkv | VideoContainer::WebM => {
                MkvExtractor::extract_from_bytes(data)
            }
            VideoContainer::Avi => {
                RiffExtractor::extract_from_bytes(data)
            }
            VideoContainer::Other(_) => {
                // Try MP4 first, then MKV, then AVI
                Mp4Extractor::extract_from_bytes(data)
                    .or_else(|| MkvExtractor::extract_from_bytes(data))
                    .or_else(|| RiffExtractor::extract_from_bytes(data))
            }
        }
    }

    /// Enhances a `VideoItem` by inspecting header bytes.
    pub fn enrich_item(mut item: VideoItem, header_bytes: &[u8]) -> VideoItem {
        if let Some(metadata) = Self::analyze_bytes(&item.container, header_bytes) {
            item = item.with_metadata(metadata);
        }
        item
    }
}
