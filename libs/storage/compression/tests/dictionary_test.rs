use phone_backup_compression::{
    CompressionAlgorithm, CompressionDictionary, CompressionLevel, CompressionStrategyFactory,
    DataCategory, DictionaryTrainer, FileMetadataContext, SmartCompressionEngine,
};

#[test]
fn test_dictionary_trainer_patterns() {
    let patterns = vec![
        "SELECT * FROM users",
        "CREATE TABLE messages",
        "INTEGER PRIMARY KEY",
    ];
    let dict_data = DictionaryTrainer::create_from_patterns(&patterns);
    assert!(!dict_data.is_empty());

    let dict = CompressionDictionary::new("sql-test", DataCategory::Database, dict_data);
    assert_eq!(dict.id.as_str(), "sql-test");
    assert_eq!(dict.category, DataCategory::Database);
}

#[test]
fn test_dictionary_compression_roundtrip() {
    let dict_data =
        b"{\"user_id\": 0, \"username\": \"\", \"email\": \"\", \"created_at\": \"\"}".to_vec();
    let dict = CompressionDictionary::new("json-dict", DataCategory::Document, dict_data);

    let strategy = CompressionStrategyFactory::create_with_dictionary(
        CompressionAlgorithm::Zstd,
        CompressionLevel::Balanced,
        dict.clone(),
    );
    assert_eq!(strategy.name(), "zstd-dict");

    let payload = b"{\"user_id\": 101, \"username\": \"alice_wonderland\", \"email\": \"alice@example.com\", \"created_at\": \"2026-09-03\"}";
    let compressed = strategy.compress(payload).unwrap();
    assert!(!compressed.is_empty());

    let decompressed = strategy.decompress(&compressed).unwrap();
    assert_eq!(decompressed, payload);
}

#[test]
fn test_smart_engine_with_android_dictionaries() {
    let engine = SmartCompressionEngine::builder()
        .with_android_dictionaries()
        .build();

    let json_msg = b"{\"name\": \"John Doe\", \"phone\": \"+62812345678\", \"timestamp\": 1725320000, \"snippet\": \"Halo, backup berhasil!\"}";
    let ctx = FileMetadataContext::new()
        .with_extension("json")
        .with_mime("application/json");

    let decision = engine.plan(json_msg, &ctx);
    assert!(decision.enabled);
    assert!(decision.dictionary_id.is_some());
    let dict_id = decision.dictionary_id.as_ref().unwrap();
    assert_eq!(dict_id.as_str(), "android-json-v1");

    let (compressed, stats) = engine.compress(json_msg, &ctx).unwrap();
    assert_eq!(stats.algorithm, CompressionAlgorithm::Zstd);

    let dict = engine.get_dictionary(dict_id).unwrap();
    let decompressed = engine
        .decompress_with_dictionary(&compressed, CompressionAlgorithm::Zstd, (*dict).clone())
        .unwrap();
    assert_eq!(decompressed, json_msg);
}
