#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataCategory {
    Media,
    Archive,
    Encrypted,
    Document,
    Database,
    Binary,
    Unknown,
}

/// Classifies file and content types to inform compression strategy.
pub struct ContentClassifier;

impl ContentClassifier {
    /// Categorizes file based on magic bytes / file signature header.
    pub fn classify_magic_bytes(header: &[u8]) -> DataCategory {
        if header.len() >= 3 && header.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return DataCategory::Media; // JPEG
        }
        if header.len() >= 4 && header.starts_with(&[0x89, b'P', b'N', b'G']) {
            return DataCategory::Media; // PNG
        }
        if header.len() >= 6 && (header.starts_with(b"GIF87a") || header.starts_with(b"GIF89a")) {
            return DataCategory::Media; // GIF
        }
        if header.len() >= 12 && &header[..4] == b"RIFF" && &header[8..12] == b"WEBP" {
            return DataCategory::Media; // WebP
        }
        if header.len() >= 8 && (&header[4..8] == b"ftyp" || &header[4..8] == b"moov") {
            return DataCategory::Media; // MP4/MOV
        }
        if header.len() >= 4 && header.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
            return DataCategory::Media; // MKV / WebM
        }
        if header.len() >= 16 && header.starts_with(b"SQLite format 3\0") {
            return DataCategory::Database; // SQLite 3
        }
        if header.len() >= 4 && header.starts_with(&[0x50, 0x4B, 0x03, 0x04]) {
            return DataCategory::Archive; // ZIP / APK / JAR
        }
        if header.len() >= 2 && header.starts_with(&[0x1F, 0x8B]) {
            return DataCategory::Archive; // GZIP
        }
        if header.len() >= 6 && header.starts_with(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C]) {
            return DataCategory::Archive; // 7z
        }
        if header.len() >= 5 && header.starts_with(b"%PDF-") {
            return DataCategory::Document; // PDF
        }
        if header.len() >= 4 && header.starts_with(b"dex\n") {
            return DataCategory::Binary; // Android DEX
        }

        DataCategory::Unknown
    }

    /// Categorizes file based on file extension.
    pub fn classify_extension(ext: &str) -> DataCategory {
        let clean_ext = ext.trim_start_matches('.').to_ascii_lowercase();
        match clean_ext.as_str() {
            "jpg" | "jpeg" | "png" | "heic" | "webp" | "gif" | "mp4" | "mkv" | "mov" | "avi"
            | "mp3" | "aac" | "m4a" | "flac" | "ogg" | "opus" => DataCategory::Media,
            "zip" | "rar" | "7z" | "tar" | "gz" | "tgz" | "bz2" | "xz" | "apk" | "jar" => {
                DataCategory::Archive
            }
            "enc" | "crypt" | "crypt12" | "crypt14" | "crypt15" => DataCategory::Encrypted,
            "txt" | "json" | "xml" | "csv" | "tsv" | "html" | "css" | "js" | "ts" | "rs" | "md"
            | "log" | "yaml" | "yml" | "vcf" | "pdf" => DataCategory::Document,
            "db" | "sqlite" | "sqlite3" | "sql" => DataCategory::Database,
            "dex" | "so" | "oat" | "bin" | "dat" => DataCategory::Binary,
            _ => DataCategory::Unknown,
        }
    }

    /// Categorizes data based on MIME type string.
    pub fn classify_mime(mime: &str) -> DataCategory {
        let clean_mime = mime.trim().to_ascii_lowercase();
        if clean_mime.starts_with("image/")
            || clean_mime.starts_with("video/")
            || clean_mime.starts_with("audio/")
        {
            return DataCategory::Media;
        }
        if clean_mime.starts_with("text/")
            || clean_mime == "application/json"
            || clean_mime == "application/xml"
            || clean_mime == "application/pdf"
            || clean_mime == "application/x-yaml"
        {
            return DataCategory::Document;
        }
        if clean_mime == "application/x-sqlite3" || clean_mime == "application/vnd.sqlite3" {
            return DataCategory::Database;
        }
        if clean_mime == "application/zip"
            || clean_mime == "application/x-rar-compressed"
            || clean_mime == "application/x-7z-compressed"
            || clean_mime == "application/vnd.android.package-archive"
        {
            return DataCategory::Archive;
        }
        DataCategory::Unknown
    }

    pub fn is_precompressed(category: DataCategory) -> bool {
        matches!(
            category,
            DataCategory::Media | DataCategory::Archive | DataCategory::Encrypted
        )
    }

    pub fn is_highly_compressible(category: DataCategory) -> bool {
        matches!(category, DataCategory::Document | DataCategory::Database)
    }
}
