use super::video_type::VideoQuality;
use serde::{Deserialize, Serialize};

/// Detailed technical video stream metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub width: u32,
    pub height: u32,
    pub duration_secs: f64,
    pub fps: Option<f32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub bitrate_bps: Option<u64>,
    pub is_hdr: bool,
    pub quality_tier: VideoQuality,
}

impl VideoMetadata {
    /// Creates a new metadata instance with calculated quality tier.
    pub fn new(width: u32, height: u32, duration_secs: f64) -> Self {
        let quality_tier = VideoQuality::from_dimensions(width, height);
        Self {
            width,
            height,
            duration_secs,
            fps: None,
            video_codec: None,
            audio_codec: None,
            bitrate_bps: None,
            is_hdr: false,
            quality_tier,
        }
    }

    /// Formats duration into human-readable HH:MM:SS or MM:SS.
    pub fn format_duration(&self) -> String {
        let total_secs = self.duration_secs.round() as u64;
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, mins, secs)
        } else {
            format!("{:02}:{:02}", mins, secs)
        }
    }

    /// Formats resolution as WxH string (e.g., 1920x1080).
    pub fn format_resolution(&self) -> String {
        if self.width > 0 && self.height > 0 {
            format!("{}x{}", self.width, self.height)
        } else {
            "-".to_string()
        }
    }
}
