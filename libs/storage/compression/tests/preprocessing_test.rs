use phone_backup_compression::DeltaEncoder;

#[test]
fn test_delta_encoder_roundtrip() {
    let original = vec![10u8, 12, 15, 20, 25, 30, 28, 26, 20, 10];
    let encoded = DeltaEncoder::encode(&original);
    assert_eq!(encoded[0], 10);
    assert_eq!(encoded[1], 2); // 12 - 10
    assert_eq!(encoded[2], 3); // 15 - 12

    let decoded = DeltaEncoder::decode(&encoded);
    assert_eq!(decoded, original);
}

#[test]
fn test_delta_encoder_wrapping_roundtrip() {
    let original = vec![250u8, 10, 5, 255, 0, 100];
    let encoded = DeltaEncoder::encode(&original);
    let decoded = DeltaEncoder::decode(&encoded);
    assert_eq!(decoded, original);
}

#[test]
fn test_delta_encoder_empty() {
    let empty: Vec<u8> = Vec::new();
    assert!(DeltaEncoder::encode(&empty).is_empty());
    assert!(DeltaEncoder::decode(&empty).is_empty());
}
