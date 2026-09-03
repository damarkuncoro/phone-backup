use phone_backup_compression::{
    BatchChunkItem, CompressionAlgorithm, FileMetadataContext, ParallelBatchCompressor,
    SmartCompressionEngine,
};
use std::sync::Arc;

#[test]
fn test_parallel_batch_compressor() {
    let engine = Arc::new(
        SmartCompressionEngine::builder()
            .with_android_dictionaries()
            .build(),
    );
    let batch_compressor = ParallelBatchCompressor::with_balanced_cpu(engine.clone());

    let ctx_json = FileMetadataContext::new()
        .with_extension("json")
        .with_mime("application/json");
    let ctx_img = FileMetadataContext::new()
        .with_extension("jpg")
        .with_mime("image/jpeg");

    let json_data = b"{\"user\":\"alice\",\"action\":\"login\",\"ip\":\"192.168.1.1\"}".repeat(20);
    let img_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x01, 0x02, 0x03, 0x04];

    let items = vec![
        BatchChunkItem::new("chunk_1", &json_data, &ctx_json),
        BatchChunkItem::new("chunk_2", &img_data, &ctx_img),
        BatchChunkItem::new("chunk_3", &json_data, &ctx_json),
    ];

    let results = batch_compressor.compress_batch(&items);
    assert_eq!(results.len(), 3);

    // chunk_1 was compressed
    let res_1 = results[0].as_ref().unwrap();
    assert_eq!(res_1.id, "chunk_1");
    assert!(res_1.compressed.len() < json_data.len());
    assert_eq!(res_1.stats.algorithm, CompressionAlgorithm::Zstd);

    // chunk_2 was skipped (image)
    let res_2 = results[1].as_ref().unwrap();
    assert_eq!(res_2.id, "chunk_2");
    assert_eq!(res_2.compressed, img_data);
    assert_eq!(res_2.stats.algorithm, CompressionAlgorithm::None);
}
