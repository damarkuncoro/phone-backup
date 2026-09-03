use crate::analysis::{BlurDetector, ImageIntegrityChecker};
use crate::exif::{ExifReader, ExifSanitizer};
use crate::model::{ExifMetadata, ImageFormat, ImageInfo};
use crate::phash::{AHashCalculator, DHashCalculator};
use crate::thumbnail::{ThumbnailGenerator, ThumbnailSize};
use anyhow::{bail, Result};
use std::collections::HashMap;

/// ProcessedImage: Comprehensive result containing analysis, hashes, metadata, and generated thumbnails.
pub struct ProcessedImage {
    pub info: ImageInfo,
    pub metadata: ExifMetadata,
    pub dhash: u64,
    pub ahash: u64,
    pub sharpness: f64,
    pub is_blurry: bool,
    pub thumbnails: HashMap<ThumbnailSize, Vec<u8>>,
}

/// ImagePipelineBuilder: Fluent builder for configuring and executing media pipelines.
pub struct ImagePipelineBuilder {
    thumbnail_sizes: Vec<ThumbnailSize>,
    thumbnail_quality: u8,
    sanitize_exif: bool,
    blur_threshold: f64,
}

impl Default for ImagePipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ImagePipelineBuilder {
    pub fn new() -> Self {
        Self {
            thumbnail_sizes: vec![ThumbnailSize::Micro, ThumbnailSize::Small, ThumbnailSize::Medium],
            thumbnail_quality: 85,
            sanitize_exif: false,
            blur_threshold: 35.0,
        }
    }

    pub fn with_thumbnail_sizes(mut self, sizes: Vec<ThumbnailSize>) -> Self {
        self.thumbnail_sizes = sizes;
        self
    }

    pub fn with_thumbnail_quality(mut self, quality: u8) -> Self {
        self.thumbnail_quality = quality.clamp(1, 100);
        self
    }

    pub fn with_exif_sanitization(mut self, sanitize: bool) -> Self {
        self.sanitize_exif = sanitize;
        self
    }

    pub fn with_blur_threshold(mut self, threshold: f64) -> Self {
        self.blur_threshold = threshold;
        self
    }

    /// Execute the configured image intelligence pipeline on raw bytes.
    pub fn process(&self, bytes: &[u8]) -> Result<ProcessedImage> {
        let format = ImageIntegrityChecker::detect_format(bytes);
        if format == ImageFormat::Unknown {
            bail!("Unsupported or unknown image format");
        }

        let img = image::load_from_memory(bytes)?;
        let (width, height) = (img.width(), img.height());

        // 1. Metadata
        let mut metadata = ExifReader::read_from_bytes(bytes).unwrap_or_default();
        if self.sanitize_exif {
            metadata = ExifSanitizer::sanitize_metadata(metadata);
        }

        // 2. Perceptual Hashes
        let dhash = DHashCalculator::compute(&img);
        let ahash = AHashCalculator::compute(&img);

        // 3. Quality & Sharpness
        let sharpness = BlurDetector::compute_sharpness(&img);
        let is_blurry = sharpness < self.blur_threshold;

        // 4. Multi-Resolution Thumbnails
        let mut thumbnails = HashMap::new();
        for &size in &self.thumbnail_sizes {
            if let Ok(thumb_bytes) = ThumbnailGenerator::generate_jpeg_bytes(&img, size, self.thumbnail_quality) {
                thumbnails.insert(size, thumb_bytes);
            }
        }

        let info = ImageInfo::new(width, height, format, bytes.len() as u64);

        Ok(ProcessedImage {
            info,
            metadata,
            dhash,
            ahash,
            sharpness,
            is_blurry,
            thumbnails,
        })
    }
}
