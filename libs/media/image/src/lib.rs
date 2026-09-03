pub mod analysis;
pub mod builder;
pub mod exif;
pub mod model;
pub mod phash;
pub mod thumbnail;

// Top-level re-exports for convenience
pub use analysis::{BlurDetector, ImageIntegrityChecker};
pub use builder::{ImagePipelineBuilder, ProcessedImage};
pub use exif::{ExifReader, ExifSanitizer};
pub use model::{CameraSettings, Dimensions, ExifMetadata, GpsCoordinates, ImageFormat, ImageInfo};
pub use phash::{AHashCalculator, DHashCalculator, PerceptualMatcher};
pub use thumbnail::{ThumbnailFactory, ThumbnailGenerator, ThumbnailPyramid, ThumbnailSize};
