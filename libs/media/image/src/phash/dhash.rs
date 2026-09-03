use image::{imageops::FilterType, DynamicImage, GenericImageView};

/// Difference Hash (dHash): Computes a 64-bit perceptual hash by comparing
/// pixel intensities horizontally across a resized 9x8 grayscale image.
pub struct DHashCalculator;

impl DHashCalculator {
    /// Compute 64-bit dHash from dynamic image.
    pub fn compute(img: &DynamicImage) -> u64 {
        // 1. Convert to grayscale and resize to 9 width x 8 height
        let gray = img.grayscale();
        let resized = gray.resize_exact(9, 8, FilterType::Triangle);

        let mut hash: u64 = 0;
        let mut bit_index = 0;

        // 2. Compare adjacent pixels horizontally: P(x) > P(x+1)
        for y in 0..8 {
            for x in 0..8 {
                let left = resized.get_pixel(x, y)[0];
                let right = resized.get_pixel(x + 1, y)[0];

                if left > right {
                    hash |= 1 << bit_index;
                }
                bit_index += 1;
            }
        }

        hash
    }

    /// Compute dHash directly from image bytes.
    pub fn compute_from_bytes(bytes: &[u8]) -> anyhow::Result<u64> {
        let img = image::load_from_memory(bytes)?;
        Ok(Self::compute(&img))
    }
}
