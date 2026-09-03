use phone_backup_compression::{
    CompressionAlgorithm, ExpertCompressor, FileMetadataContext, SmartCompressionEngine,
};

#[test]
fn test_smart_engine_compress_decompress_roundtrip() {
    let engine = SmartCompressionEngine::builder().build();
    let original = b"{\"users\": [{\"name\": \"Alice\", \"role\": \"admin\"}, {\"name\": \"Bob\", \"role\": \"user\"}]}".repeat(10);
    let ctx = FileMetadataContext::new()
        .with_extension("json")
        .with_mime("application/json");

    let (compressed, stats) = engine.compress(&original, &ctx).unwrap();
    assert!(compressed.len() < original.len());
    assert!(stats.saved_bytes > 0);
    assert_eq!(stats.algorithm, CompressionAlgorithm::Zstd);

    let decompressed = engine
        .decompress(&compressed, CompressionAlgorithm::Zstd)
        .unwrap();
    assert_eq!(decompressed, original);
}

#[test]
fn test_expert_compressor_legacy_facade() {
    let data = b"expert compression test data expert compression test data";
    let compressed = ExpertCompressor::compress(data, CompressionAlgorithm::Zstd).unwrap();
    assert!(compressed.len() < data.len());

    let decompressed =
        ExpertCompressor::decompress(&compressed, CompressionAlgorithm::Zstd).unwrap();
    assert_eq!(data.to_vec(), decompressed);

    assert!(ExpertCompressor::should_compress("application/json"));
    assert!(!ExpertCompressor::should_compress("image/jpeg"));
}
