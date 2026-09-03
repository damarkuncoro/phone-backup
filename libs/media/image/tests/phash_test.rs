use image::{DynamicImage, Rgb, RgbImage};
use phone_backup_image::{AHashCalculator, DHashCalculator, PerceptualMatcher};

fn create_test_gradient_image(width: u32, height: u32, reverse: bool) -> DynamicImage {
    let mut img = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let px = if reverse { width - 1 - x } else { x };
            let r = (px * 255 / width) as u8;
            let g = (y * 255 / height) as u8;
            let b = ((px + y) * 128 / (width + height)) as u8;
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    DynamicImage::ImageRgb8(img)
}

#[test]
fn test_dhash_and_ahash_consistency() {
    let img1 = create_test_gradient_image(200, 200, false);
    let dhash1 = DHashCalculator::compute(&img1);
    let ahash1 = AHashCalculator::compute(&img1);

    // Resized version of the same image (simulating a thumbnail or compressed version)
    let img2 = create_test_gradient_image(100, 100, false);
    let dhash2 = DHashCalculator::compute(&img2);
    let ahash2 = AHashCalculator::compute(&img2);

    // Same gradient image at different resolutions should have 0 or very low Hamming distance
    let dhash_distance = PerceptualMatcher::hamming_distance(dhash1, dhash2);
    let ahash_distance = PerceptualMatcher::hamming_distance(ahash1, ahash2);

    assert!(dhash_distance <= 2, "dHash difference: {}", dhash_distance);
    assert!(ahash_distance <= 2, "aHash difference: {}", ahash_distance);
    assert!(PerceptualMatcher::is_near_duplicate(dhash1, dhash2, 5));
    assert!(PerceptualMatcher::similarity(dhash1, dhash2) >= 0.95);
}

#[test]
fn test_distinct_images_have_large_distance() {
    let img_forward = create_test_gradient_image(100, 100, false);
    let img_reverse = create_test_gradient_image(100, 100, true);

    let hash_fwd = DHashCalculator::compute(&img_forward);
    let hash_rev = DHashCalculator::compute(&img_reverse);

    let distance = PerceptualMatcher::hamming_distance(hash_fwd, hash_rev);
    assert!(distance >= 32, "Expected large bit difference between opposing gradients, got {}", distance);
    assert!(!PerceptualMatcher::is_near_duplicate(hash_fwd, hash_rev, 5));
}
