use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThumbnailSize {
    /// 64x64 micro icon / blurhash placeholder
    Micro,
    /// 256x256 standard gallery grid thumbnail
    Small,
    /// 512x512 high-DPI thumbnail preview
    Medium,
    /// 1024x1024 full screen preview
    Large,
    /// Custom maximum bounding box width and height
    Custom(u32, u32),
}

impl ThumbnailSize {
    pub fn max_dimensions(&self) -> (u32, u32) {
        match self {
            Self::Micro => (64, 64),
            Self::Small => (256, 256),
            Self::Medium => (512, 512),
            Self::Large => (1024, 1024),
            Self::Custom(w, h) => (*w, *h),
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            Self::Micro => "micro",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Custom(_, _) => "custom",
        }
    }
}
