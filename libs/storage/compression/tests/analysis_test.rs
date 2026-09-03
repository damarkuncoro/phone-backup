use phone_backup_compression::{ContentClassifier, DataCategory, EntropyDetector, SampleProbe};

#[test]
fn test_classifier_by_magic_bytes() {
    // JPEG magic bytes
    let jpeg_header = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
    assert_eq!(
        ContentClassifier::classify_magic_bytes(&jpeg_header),
        DataCategory::Media
    );

    // PNG magic bytes
    let png_header = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    assert_eq!(
        ContentClassifier::classify_magic_bytes(&png_header),
        DataCategory::Media
    );

    // SQLite header
    let sqlite_header = b"SQLite format 3\0\x10\x00\x01\x01";
    assert_eq!(
        ContentClassifier::classify_magic_bytes(sqlite_header),
        DataCategory::Database
    );

    // ZIP / APK header
    let zip_header = [0x50, 0x4B, 0x03, 0x04, 0x14, 0x00];
    assert_eq!(
        ContentClassifier::classify_magic_bytes(&zip_header),
        DataCategory::Archive
    );

    // PDF header
    let pdf_header = b"%PDF-1.7\n%...";
    assert_eq!(
        ContentClassifier::classify_magic_bytes(pdf_header),
        DataCategory::Document
    );
}

#[test]
fn test_classifier_by_extension() {
    assert_eq!(
        ContentClassifier::classify_extension("jpg"),
        DataCategory::Media
    );
    assert_eq!(
        ContentClassifier::classify_extension(".mp4"),
        DataCategory::Media
    );
    assert_eq!(
        ContentClassifier::classify_extension("zip"),
        DataCategory::Archive
    );
    assert_eq!(
        ContentClassifier::classify_extension("json"),
        DataCategory::Document
    );
    assert_eq!(
        ContentClassifier::classify_extension("sqlite"),
        DataCategory::Database
    );
}

#[test]
fn test_classifier_by_mime() {
    assert_eq!(
        ContentClassifier::classify_mime("image/jpeg"),
        DataCategory::Media
    );
    assert_eq!(
        ContentClassifier::classify_mime("application/json"),
        DataCategory::Document
    );
    assert_eq!(
        ContentClassifier::classify_mime("application/x-sqlite3"),
        DataCategory::Database
    );
}

#[test]
fn test_entropy_detector_low_entropy() {
    let repetitive_data = b"AAAAABBBBBCCCCCDDDDDAAAAABBBBBCCCCCDDDDD".repeat(50);
    let entropy = EntropyDetector::calculate_entropy(&repetitive_data);
    assert!(entropy < 3.0, "Entropy was {entropy}, expected < 3.0");
    assert!(!EntropyDetector::is_high_entropy(&repetitive_data, 7.5));
}

#[test]
fn test_entropy_detector_high_entropy() {
    let mut random_like = Vec::with_capacity(256 * 10);
    for _ in 0..10 {
        for b in 0..=255u8 {
            random_like.push(b);
        }
    }
    let entropy = EntropyDetector::calculate_entropy(&random_like);
    assert!(entropy >= 7.9, "Entropy was {entropy}, expected >= 7.9");
    assert!(EntropyDetector::is_high_entropy(&random_like, 7.5));
}

#[test]
fn test_sample_probe_worth_compressing() {
    let compressible_data = b"Repeated text for phone backup testing. ".repeat(100);
    assert!(SampleProbe::is_worth_compressing(
        &compressible_data,
        64 * 1024,
        0.10
    ));
}
