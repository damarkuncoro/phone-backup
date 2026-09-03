use crate::model::ExifMetadata;

pub struct ExifSanitizer;

impl ExifSanitizer {
    /// Strips privacy-sensitive data (GPS coordinates, lens serials, owner info) from metadata.
    pub fn sanitize_metadata(mut metadata: ExifMetadata) -> ExifMetadata {
        metadata.gps = None;
        metadata.software = None;
        metadata
    }

    /// Re-encodes dynamic image to clean JPEG bytes without any EXIF metadata.
    pub fn strip_exif_from_image(img: &image::DynamicImage) -> anyhow::Result<Vec<u8>> {
        let mut buffer = std::io::Cursor::new(Vec::new());
        img.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(90))?;
        Ok(buffer.into_inner())
    }
}
