use super::audio_format::AudioFormat;
use super::category::AudioCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioStreamInfo {
    pub duration_seconds: f64,
    pub bitrate_kbps: u32,
    pub sample_rate_hz: u32,
    pub channels: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub stream_info: AudioStreamInfo,
    pub format: Option<AudioFormat>,
    pub category: Option<AudioCategory>,
}

impl AudioMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
