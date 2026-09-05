use domain::{FileEntry, ScanCategory, ScanCategorySummary};
use std::collections::BTreeMap;

/// Intelligent file category classifier based on path, extension, and MIME type.
pub struct FileClassifier;

impl FileClassifier {
    /// Determines the ScanCategory for a given file entry.
    pub fn classify(file: &FileEntry) -> ScanCategory {
        let path_lower = file.path.to_lowercase();
        let mime_lower = file.mime_type.to_lowercase();
        let ext = file
            .path
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        // 1. WhatsApp takes precedence for app-specific chat media
        if path_lower.contains("whatsapp") || path_lower.contains("com.whatsapp") {
            return ScanCategory::WhatsApp;
        }

        // 2. Photos & Images
        if mime_lower.starts_with("image/")
            || matches!(
                ext.as_str(),
                "jpg" | "jpeg" | "png" | "heic" | "heif" | "webp" | "dng" | "raw" | "gif" | "svg" | "bmp"
            )
        {
            return ScanCategory::Photos;
        }

        // 3. Videos
        if mime_lower.starts_with("video/")
            || matches!(
                ext.as_str(),
                "mp4" | "mkv" | "mov" | "avi" | "3gp" | "ts" | "webm" | "m4v" | "flv"
            )
        {
            return ScanCategory::Videos;
        }

        // 4. Audio & Voice Notes
        if mime_lower.starts_with("audio/")
            || matches!(
                ext.as_str(),
                "mp3" | "m4a" | "aac" | "opus" | "ogg" | "flac" | "wav" | "amr" | "wma" | "mid"
            )
        {
            return ScanCategory::Audio;
        }

        // 5. APKs & App Bundles
        if matches!(ext.as_str(), "apk" | "xapk" | "apks" | "aab") {
            return ScanCategory::Apks;
        }

        // 6. Documents
        if mime_lower.contains("pdf")
            || mime_lower.contains("document")
            || mime_lower.contains("spreadsheet")
            || mime_lower.contains("text")
            || matches!(
                ext.as_str(),
                "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "csv" | "epub" | "rtf"
            )
        {
            return ScanCategory::Documents;
        }

        // 7. General Downloads
        if path_lower.contains("/download/") || path_lower.contains("/downloads/") {
            return ScanCategory::Downloads;
        }

        // 8. System Paths
        if path_lower.starts_with("/system") || path_lower.starts_with("/data") {
            return ScanCategory::System;
        }

        ScanCategory::Other
    }

    /// Aggregates a list of FileEntries into category summaries.
    pub fn summarize(files: &[FileEntry]) -> BTreeMap<ScanCategory, ScanCategorySummary> {
        let mut map = BTreeMap::new();
        for file in files {
            let cat = Self::classify(file);
            let entry = map.entry(cat).or_insert_with(ScanCategorySummary::default);
            entry.file_count += 1;
            entry.total_bytes += file.size_bytes;
        }
        map
    }
}
