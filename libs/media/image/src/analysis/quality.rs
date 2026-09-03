use image::DynamicImage;

/// BlurDetector: Computes image sharpness using Laplacian operator variance.
/// Low variance indicates out-of-focus or motion-blurred photo.
pub struct BlurDetector;

impl BlurDetector {
    /// Compute sharpness score (variance of Laplacian).
    /// Typical thresholds:
    /// - > 100.0: Crisp and sharp
    /// - 30.0 - 100.0: Acceptable sharpness
    /// - < 30.0: Blurry / out of focus
    pub fn compute_sharpness(img: &DynamicImage) -> f64 {
        let gray = img.to_luma8();
        let (width, height) = gray.dimensions();

        if width < 3 || height < 3 {
            return 0.0;
        }

        let mut laplacian_values = Vec::with_capacity(((width - 2) * (height - 2)) as usize);
        let mut sum: f64 = 0.0;

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let center = gray.get_pixel(x, y)[0] as f64;
                let top = gray.get_pixel(x, y - 1)[0] as f64;
                let bottom = gray.get_pixel(x, y + 1)[0] as f64;
                let left = gray.get_pixel(x - 1, y)[0] as f64;
                let right = gray.get_pixel(x + 1, y)[0] as f64;

                let lap = (4.0 * center) - top - bottom - left - right;
                sum += lap;
                laplacian_values.push(lap);
            }
        }

        let count = laplacian_values.len() as f64;
        let mean = sum / count;

        let variance = laplacian_values
            .iter()
            .map(|&val| {
                let diff = val - mean;
                diff * diff
            })
            .sum::<f64>()
            / count;

        variance
    }

    /// Check if image is blurry according to threshold (default threshold: 35.0).
    pub fn is_blurry(img: &DynamicImage, threshold: f64) -> bool {
        Self::compute_sharpness(img) < threshold
    }
}
