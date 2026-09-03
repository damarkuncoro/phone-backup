use image::{DynamicImage, Rgb, RgbImage};
use phone_backup_image::{BlurDetector, ImageIntegrityChecker, ImagePipelineBuilder};

#[test]
fn test_blur_detector_sharp_vs_flat() {
    // 1. High contrast / sharp checkered image
    let mut sharp_img = RgbImage::new(100, 100);
    for y in 0..100 {
        for x in 0..100 {
            let color = if ((x / 10) + (y / 10)) % 2 == 0 { 255 } else { 0 };
            sharp_img.put_pixel(x, y, Rgb([color, color, color]));
        }
    }
    let sharp_dyn = DynamicImage::ImageRgb8(sharp_img);
    let sharp_score = BlurDetector::compute_sharpness(&sharp_dyn);

    // 2. Uniform / flat image (no edges, represents total blur)
    let flat_img = DynamicImage::ImageRgb8(RgbImage::from_pixel(100, 100, Rgb([128, 128, 128])));
    let flat_score = BlurDetector::compute_sharpness(&flat_img);

    assert!(sharp_score > 500.0, "Sharp score was {}", sharp_score);
    assert_eq!(flat_score, 0.0);
    assert!(BlurDetector::is_blurry(&flat_img, 30.0));
    assert!(!BlurDetector::is_blurry(&sharp_dyn, 30.0));
}

#[test]
fn test_pipeline_builder_full_processing() {
    let mut img = RgbImage::new(300, 200);
    for y in 0..200 {
        for x in 0..300 {
            img.put_pixel(x, y, Rgb([(x % 256) as u8, (y % 256) as u8, 150]));
        }
    }
    let dyn_img = DynamicImage::ImageRgb8(img);
    let mut jpeg_bytes = std::io::Cursor::new(Vec::new());
    dyn_img.write_to(&mut jpeg_bytes, image::ImageOutputFormat::Jpeg(90)).unwrap();
    let bytes = jpeg_bytes.into_inner();

    assert!(ImageIntegrityChecker::check_integrity(&bytes));

    let pipeline = ImagePipelineBuilder::new()
        .with_thumbnail_quality(80)
        .with_blur_threshold(20.0);

    let result = pipeline.process(&bytes).expect("Pipeline execution failed");

    assert_eq!(result.info.dimensions.width, 300);
    assert_eq!(result.info.dimensions.height, 200);
    assert!(result.thumbnails.len() >= 2);
    assert!(result.dhash > 0);
}
