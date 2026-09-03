use image::{DynamicImage, Rgb, RgbImage};
use phone_backup_image::{ThumbnailFactory, ThumbnailGenerator, ThumbnailSize};

fn create_sample_image(w: u32, h: u32) -> DynamicImage {
    DynamicImage::ImageRgb8(RgbImage::from_pixel(w, h, Rgb([100, 150, 200])))
}

#[test]
fn test_thumbnail_generation_dimensions() {
    let img = create_sample_image(1920, 1080);

    let micro = ThumbnailGenerator::generate_thumbnail(&img, ThumbnailSize::Micro);
    assert!(micro.width() <= 64 && micro.height() <= 64);

    let small = ThumbnailGenerator::generate_thumbnail(&img, ThumbnailSize::Small);
    assert!(small.width() <= 256 && small.height() <= 256);

    let medium = ThumbnailGenerator::generate_thumbnail(&img, ThumbnailSize::Medium);
    assert!(medium.width() <= 512 && medium.height() <= 512);

    let large = ThumbnailGenerator::generate_thumbnail(&img, ThumbnailSize::Large);
    assert!(large.width() <= 1024 && large.height() <= 1024);
}

#[test]
fn test_thumbnail_factory_pyramid() {
    let img = create_sample_image(800, 600);
    let pyramid = ThumbnailFactory::create_standard_pyramid(&img, 85).expect("Failed to create pyramid");

    assert!(pyramid.thumbnails.contains_key(&ThumbnailSize::Micro));
    assert!(pyramid.thumbnails.contains_key(&ThumbnailSize::Small));
    assert!(pyramid.thumbnails.contains_key(&ThumbnailSize::Medium));

    for (&size, bytes) in &pyramid.thumbnails {
        assert!(!bytes.is_empty(), "Thumbnail for {:?} was empty", size);
        let decoded = image::load_from_memory(bytes).expect("Failed to decode generated JPEG thumbnail");
        let (max_w, max_h) = size.max_dimensions();
        assert!(decoded.width() <= max_w && decoded.height() <= max_h);
    }
}
