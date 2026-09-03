use phone_backup_image::{ExifMetadata, ExifReader, ExifSanitizer, GpsCoordinates};

#[test]
fn test_exif_sanitizer_removes_sensitive_gps() {
    let mut metadata = ExifMetadata::default();
    metadata.gps = Some(GpsCoordinates::new(-6.2088, 106.8456, Some(12.5)));
    metadata.software = Some("Vivo Camera App v4.2".to_string());
    metadata.camera.make = Some("vivo".to_string());
    metadata.camera.model = Some("V2317".to_string());

    assert!(metadata.has_gps());
    assert!(metadata.has_camera_info());

    let sanitized = ExifSanitizer::sanitize_metadata(metadata);
    assert!(!sanitized.has_gps());
    assert!(sanitized.software.is_none());
    assert_eq!(sanitized.camera.model.as_deref(), Some("V2317"));
}

#[test]
fn test_exif_reader_handles_empty_or_non_exif_bytes() {
    let dummy_bytes = vec![0u8; 64];
    let res = ExifReader::read_from_bytes(&dummy_bytes).expect("Should return default metadata without error");
    assert!(!res.has_gps());
    assert!(!res.has_camera_info());
}
