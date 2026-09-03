use crate::model::ImageFormat;

pub struct ImageIntegrityChecker;

impl ImageIntegrityChecker {
    /// Detect format from magic bytes without full decoding.
    pub fn detect_format(bytes: &[u8]) -> ImageFormat {
        if bytes.len() < 4 {
            return ImageFormat::Unknown;
        }

        if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
            ImageFormat::Jpeg
        } else if bytes.starts_with(&[0x89, b'P', b'N', b'G']) {
            ImageFormat::Png
        } else if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
            ImageFormat::WebP
        } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
            ImageFormat::Gif
        } else {
            ImageFormat::Unknown
        }
    }

    /// Verifies that image byte stream can be successfully decoded without corruption.
    pub fn verify_decodable(bytes: &[u8]) -> bool {
        image::load_from_memory(bytes).is_ok()
    }

    /// Quick integrity check: verifies format and basic EOF markers.
    pub fn check_integrity(bytes: &[u8]) -> bool {
        let format = Self::detect_format(bytes);
        match format {
            ImageFormat::Jpeg => bytes.len() >= 4 && bytes.starts_with(&[0xFF, 0xD8]) && bytes.ends_with(&[0xFF, 0xD9]),
            ImageFormat::Png => bytes.len() >= 8 && bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]),
            ImageFormat::WebP => bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP",
            ImageFormat::Gif => bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a"),
            ImageFormat::Unknown => false,
        }
    }
}
