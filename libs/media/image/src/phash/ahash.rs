use image::{imageops::FilterType, DynamicImage, GenericImageView};

/// Average Hash (aHash): Computes a 64-bit perceptual hash by comparing
/// pixel intensities against the overall mean luminance on an 8x8 image.
pub struct AHashCalculator;

impl AHashCalculator {
    /// Compute 64-bit aHash from dynamic image.
    pub fn compute(img: &DynamicImage) -> u64 {
        // 1. Grayscale and resize to 8x8
        let gray = img.grayscale();
        let resized = gray.resize_exact(8, 8, FilterType::Triangle);

        // 2. Compute mean luminance
        let mut sum: u64 = 0;
        for y in 0..8 {
            for x in 0..8 {
                sum += resized.get_pixel(x, y)[0] as u64;
            }
        }
        let avg = (sum / 64) as u8;

        // 3. Set bit to 1 if pixel >= average
        let mut hash: u64 = 0;
        let mut bit_index = 0;

        for y in 0..8 {
            for x in 0..8 {
                if resized.get_pixel(x, y)[0] >= avg {
                    hash |= 1 << bit_index;
                }
                bit_index += 1;
            }
        }

        hash
    }

    /// Compute aHash directly from image bytes.
    pub fn compute_from_bytes(bytes: &[u8]) -> anyhow::Result<u64> {
        let img = image::load_from_memory(bytes)?;
        Ok(Self::compute(&img))
    }
}
