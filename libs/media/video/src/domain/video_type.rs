use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported video container formats.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VideoContainer {
    Mp4,
    Mov,
    Mkv,
    WebM,
    Avi,
    ThreeGP,
    Other(String),
}

impl VideoContainer {
    /// Detects container format from file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "mp4" | "m4v" => Self::Mp4,
            "mov" | "qt" => Self::Mov,
            "mkv" => Self::Mkv,
            "webm" => Self::WebM,
            "avi" => Self::Avi,
            "3gp" | "3g2" => Self::ThreeGP,
            other => Self::Other(other.to_string()),
        }
    }

    /// Returns human-readable label.
    pub fn display_name(&self) -> &str {
        match self {
            Self::Mp4 => "MP4 Video",
            Self::Mov => "QuickTime MOV",
            Self::Mkv => "Matroska MKV",
            Self::WebM => "WebM Video",
            Self::Avi => "AVI Video",
            Self::ThreeGP => "3GPP Video",
            Self::Other(ext) => ext.as_str(),
        }
    }
}

impl fmt::Display for VideoContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Video resolution and quality classification tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VideoQuality {
    Uhd4K,
    Qhd2K,
    Fhd1080p,
    Hd720p,
    Sd480p,
    LowRes,
    Unknown,
}

impl VideoQuality {
    /// Determines quality tier based on pixel dimensions.
    pub fn from_dimensions(width: u32, height: u32) -> Self {
        let max_dim = width.max(height);
        let min_dim = width.min(height);

        if max_dim >= 3840 || min_dim >= 2160 {
            Self::Uhd4K
        } else if max_dim >= 2560 || min_dim >= 1440 {
            Self::Qhd2K
        } else if max_dim >= 1920 || min_dim >= 1080 {
            Self::Fhd1080p
        } else if max_dim >= 1280 || min_dim >= 720 {
            Self::Hd720p
        } else if max_dim >= 640 || min_dim >= 480 {
            Self::Sd480p
        } else if max_dim > 0 {
            Self::LowRes
        } else {
            Self::Unknown
        }
    }

    /// Returns label of the quality tier.
    pub fn display_name(&self) -> &str {
        match self {
            Self::Uhd4K => "4K UHD (2160p)",
            Self::Qhd2K => "2K QHD (1440p)",
            Self::Fhd1080p => "Full HD (1080p)",
            Self::Hd720p => "HD (720p)",
            Self::Sd480p => "SD (480p)",
            Self::LowRes => "Low Resolution",
            Self::Unknown => "Unknown Quality",
        }
    }
}

impl fmt::Display for VideoQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
