use phone_backup_compression::{
    CompressionAlgorithm, CompressionPolicy, FileMetadataContext, SmartCompressionEngine,
};

#[test]
fn test_builder_fluent_configuration() {
    let engine = SmartCompressionEngine::builder()
        .with_policy(CompressionPolicy::Adaptive)
        .with_entropy_threshold(7.2)
        .with_entropy_check(true)
        .with_sample_probe(true)
        .with_chunk_size(2 * 1024 * 1024)
        .build();

    let text_data = b"Some application log database text content. ".repeat(200);
    let ctx = FileMetadataContext::new()
        .with_extension("log")
        .with_mime("text/plain");

    let decision = engine.plan(&text_data, &ctx);
    assert!(decision.enabled);
    assert_eq!(decision.algorithm, CompressionAlgorithm::Zstd);
    assert_eq!(decision.chunk_size, 2 * 1024 * 1024);
}

#[test]
fn test_builder_skips_precompressed_media() {
    let engine = SmartCompressionEngine::builder()
        .with_policy(CompressionPolicy::Adaptive)
        .build();

    let dummy_image_bytes = vec![0xAB; 2048];
    let ctx = FileMetadataContext::new()
        .with_extension("jpg")
        .with_mime("image/jpeg");

    let decision = engine.plan(&dummy_image_bytes, &ctx);
    assert!(!decision.enabled);
}

#[test]
fn test_builder_skips_high_entropy_bytes() {
    let engine = SmartCompressionEngine::builder()
        .with_policy(CompressionPolicy::Adaptive)
        .with_entropy_threshold(7.5)
        .build();

    let mut high_entropy = Vec::new();
    for _ in 0..10 {
        for b in 0..=255u8 {
            high_entropy.push(b);
        }
    }

    let ctx = FileMetadataContext::new().with_extension("unknown");
    let decision = engine.plan(&high_entropy, &ctx);
    assert!(!decision.enabled);
}
