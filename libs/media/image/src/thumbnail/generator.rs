use super::size::ThumbnailSize;
use anyhow::Result;
use image::{imageops::FilterType, DynamicImage, ImageOutputFormat};
use std::io::Cursor;

pub struct ThumbnailGenerator;

impl ThumbnailGenerator {
    /// Generate a scaled thumbnail preserving aspect ratio within bounding box.
    pub fn generate_thumbnail(img: &DynamicImage, size: ThumbnailSize) -> DynamicImage {
        let (max_w, max_h) = size.max_dimensions();
        img.resize(max_w, max_h, FilterType::Lanczos3)
    }

    /// Generate thumbnail and encode directly to JPEG bytes with specified quality (1-100).
    pub fn generate_jpeg_bytes(img: &DynamicImage, size: ThumbnailSize, quality: u8) -> Result<Vec<u8>> {
        let thumb = Self::generate_thumbnail(img, size);
        let mut buffer = Cursor::new(Vec::new());
        thumb.write_to(&mut buffer, ImageOutputFormat::Jpeg(quality))?;
        Ok(buffer.into_inner())
    }

    /// Generate thumbnail directly from input image bytes.
    pub fn generate_from_bytes(bytes: &[u8], size: ThumbnailSize, quality: u8) -> Result<Vec<u8>> {
        let img = image::load_from_memory(bytes)?;
        Self::generate_jpeg_bytes(&img, size, quality)
    }
}
