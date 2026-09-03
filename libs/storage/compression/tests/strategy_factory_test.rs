use phone_backup_compression::{
    CompressionAlgorithm, CompressionLevel, CompressionStrategyFactory,
};

#[test]
fn test_factory_creates_no_compression_strategy() {
    let strategy =
        CompressionStrategyFactory::create(CompressionAlgorithm::None, CompressionLevel::Fast);
    assert_eq!(strategy.name(), "none");

    let input = b"Hello, World!";
    let compressed = strategy.compress(input).unwrap();
    assert_eq!(compressed, input);

    let decompressed = strategy.decompress(&compressed).unwrap();
    assert_eq!(decompressed, input);
}

#[test]
fn test_factory_creates_zstd_strategy_with_levels() {
    let fast_strat =
        CompressionStrategyFactory::create(CompressionAlgorithm::Zstd, CompressionLevel::Fast);
    assert_eq!(fast_strat.name(), "zstd");

    let max_strat =
        CompressionStrategyFactory::create(CompressionAlgorithm::Zstd, CompressionLevel::Maximum);
    assert_eq!(max_strat.name(), "zstd");

    let sample = b"Phone backup compression test content repeated. Phone backup compression test content repeated.";
    let compressed = fast_strat.compress(sample).unwrap();
    assert!(compressed.len() < sample.len());

    let decompressed = max_strat.decompress(&compressed).unwrap();
    assert_eq!(decompressed, sample);
}

#[test]
fn test_factory_default_creation() {
    let strategy = CompressionStrategyFactory::create_default(CompressionAlgorithm::Zstd);
    assert_eq!(strategy.name(), "zstd");
}
