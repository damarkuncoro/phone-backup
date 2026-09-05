use crate::domain::TelegramMediaType;

/// Resolves standard Telegram file storage paths on Android.
pub struct TelegramPathResolver;

impl TelegramPathResolver {
    /// Returns known root paths where Telegram stores data on Android storage.
    pub fn get_search_roots() -> Vec<&'static str> {
        vec![
            "/sdcard/Telegram",
            "/storage/emulated/0/Telegram",
            "/sdcard/Android/data/org.telegram.messenger/files/Telegram",
            "/storage/emulated/0/Android/data/org.telegram.messenger/files/Telegram",
            "/sdcard/Android/media/org.telegram.messenger",
            "/storage/emulated/0/Android/media/org.telegram.messenger",
        ]
    }

    /// Determines media type based on relative Telegram folder name and file extension.
    pub fn classify_path(path: &str) -> TelegramMediaType {
        let p_lower = path.to_lowercase();
        if p_lower.contains("telegram audio") || p_lower.contains("voice") {
            TelegramMediaType::VoiceNote
        } else if p_lower.contains("telegram video") || p_lower.contains("video note") {
            if p_lower.contains("round") || p_lower.contains("note") {
                TelegramMediaType::VideoNote
            } else {
                TelegramMediaType::Video
            }
        } else if p_lower.contains("telegram images") || p_lower.contains("photos") {
            TelegramMediaType::Photo
        } else if p_lower.contains("telegram documents") {
            TelegramMediaType::Document
        } else {
            let ext = std::path::Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            TelegramMediaType::from_mime_or_ext(ext)
        }
    }
}
