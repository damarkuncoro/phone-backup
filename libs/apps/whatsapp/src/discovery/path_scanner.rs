use crate::model::MediaCategory;

pub struct WhatsAppPathScanner;

impl WhatsAppPathScanner {
    pub const SCOPED_STORAGE_BASE: &'static str = "/storage/emulated/0/Android/media/com.whatsapp/WhatsApp";
    pub const SCOPED_BUSINESS_BASE: &'static str = "/storage/emulated/0/Android/media/com.whatsapp.w4b/WhatsApp Business";
    pub const LEGACY_STORAGE_BASE: &'static str = "/storage/emulated/0/WhatsApp";

    /// Returns candidate base directories in order of modern preference.
    pub fn candidate_roots() -> &'static [&'static str] {
        &[
            Self::SCOPED_STORAGE_BASE,
            Self::SCOPED_BUSINESS_BASE,
            Self::LEGACY_STORAGE_BASE,
        ]
    }

    /// Categorizes WhatsApp media folder based on relative or absolute path.
    pub fn categorize_path(path: &str) -> MediaCategory {
        if path.contains("WhatsApp Voice Notes") {
            MediaCategory::VoiceNotes
        } else if path.contains("WhatsApp Images") {
            MediaCategory::Images
        } else if path.contains("WhatsApp Video") {
            MediaCategory::Video
        } else if path.contains("WhatsApp Audio") {
            MediaCategory::Audio
        } else if path.contains("WhatsApp Documents") {
            MediaCategory::Documents
        } else if path.contains("WhatsApp Stickers") {
            MediaCategory::Stickers
        } else if path.contains("WhatsApp Animated Gifs") {
            MediaCategory::Gifs
        } else if path.contains("WhatsApp Profile Photos") {
            MediaCategory::ProfilePhotos
        } else {
            MediaCategory::Unknown
        }
    }
}
