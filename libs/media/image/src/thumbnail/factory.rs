use super::generator::ThumbnailGenerator;
use super::size::ThumbnailSize;
use anyhow::Result;
use image::DynamicImage;
use std::collections::HashMap;

/// ThumbnailPyramid: Multi-resolution thumbnail collection for gallery rendering.
pub struct ThumbnailPyramid {
    pub thumbnails: HashMap<ThumbnailSize, Vec<u8>>,
}

/// ThumbnailFactory: Factory pattern for generating thumbnail tiers.
pub struct ThumbnailFactory;

impl ThumbnailFactory {
    /// Create standard gallery thumbnail tiers (Micro, Small, Medium) from dynamic image.
    pub fn create_standard_pyramid(img: &DynamicImage, quality: u8) -> Result<ThumbnailPyramid> {
        let mut map = HashMap::new();

        let sizes = [ThumbnailSize::Micro, ThumbnailSize::Small, ThumbnailSize::Medium];
        for size in sizes {
            let bytes = ThumbnailGenerator::generate_jpeg_bytes(img, size, quality)?;
            map.insert(size, bytes);
        }

        Ok(ThumbnailPyramid { thumbnails: map })
    }

    /// Create standard gallery pyramid directly from raw image bytes.
    pub fn create_from_bytes(bytes: &[u8], quality: u8) -> Result<ThumbnailPyramid> {
        let img = image::load_from_memory(bytes)?;
        Self::create_standard_pyramid(&img, quality)
    }
}
