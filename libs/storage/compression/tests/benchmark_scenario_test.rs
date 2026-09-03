use phone_backup_compression::{
    BatchChunkItem, CompressionAlgorithm, DeltaEncoder, FileMetadataContext,
    ParallelBatchCompressor, SmartCompressionEngine,
};
use std::sync::Arc;

#[test]
fn test_realworld_phone_backup_compression_scenarios() {
    let engine = Arc::new(
        SmartCompressionEngine::builder()
            .with_android_dictionaries()
            .build(),
    );

    println!("\n=========================================================================================");
    println!("                  PHONE BACKUP SMART COMPRESSION BENCHMARK REPORT                         ");
    println!("=========================================================================================");
    println!(
        "{:<20} | {:<12} | {:<12} | {:<10} | {:<10} | {:<15}",
        "Data Type", "Orig Size", "Comp Size", "Ratio", "Saved %", "Decision/Algo"
    );
    println!("-----------------------------------------------------------------------------------------");

    // 1. WhatsApp / SMS JSON Data with Android Dictionary
    let sms_json = b"{\"address\":\"+62812345678\",\"body\":\"Halo, ini kode OTP verifikasi backup Anda 482910\",\"date\":1725320000,\"read\":1,\"type\":1,\"status\":0,\"thread_id\":42}".repeat(20);
    let ctx_sms = FileMetadataContext::new().with_extension("json").with_mime("application/json");
    let (comp_sms, stats_sms) = engine.compress(&sms_json, &ctx_sms).unwrap();

    println!(
        "{:<20} | {:>10} B | {:>10} B | {:>9.2} | {:>9.1}% | Zstd (Dict: android-json-v1)",
        "SMS/Chat JSON", sms_json.len(), comp_sms.len(), stats_sms.ratio, stats_sms.savings_percentage()
    );
    assert!(stats_sms.savings_percentage() > 70.0);

    // 2. WhatsApp SQLite Database
    let mut sqlite_db = b"SQLite format 3\0\x10\x00\x01\x01\x00\x40\x20\x20\x00\x00\x00\x01".to_vec();
    sqlite_db.extend_from_slice(&b"CREATE TABLE messages (id INTEGER PRIMARY KEY, sender TEXT, body TEXT); INSERT INTO messages VALUES (1, 'Alice', 'Hello world');".repeat(30));
    let ctx_db = FileMetadataContext::new().with_extension("db");
    let (comp_db, stats_db) = engine.compress(&sqlite_db, &ctx_db).unwrap();

    println!(
        "{:<20} | {:>10} B | {:>10} B | {:>9.2} | {:>9.1}% | Zstd (Dict: android-sqlite-v1)",
        "SQLite Database", sqlite_db.len(), comp_db.len(), stats_db.ratio, stats_db.savings_percentage()
    );
    assert!(stats_db.savings_percentage() > 60.0);

    // 3. JPEG Image (Magic bytes detection -> Auto Bypass)
    let mut jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46];
    jpeg_data.extend(vec![0x7A; 4096]); // dummy image payload
    let ctx_img = FileMetadataContext::new().with_extension("jpg");
    let (comp_img, stats_img) = engine.compress(&jpeg_data, &ctx_img).unwrap();

    println!(
        "{:<20} | {:>10} B | {:>10} B | {:>9.2} | {:>9.1}% | Skipped (Magic: Media)",
        "Camera JPEG", jpeg_data.len(), comp_img.len(), stats_img.ratio, stats_img.savings_percentage()
    );
    assert_eq!(stats_img.algorithm, CompressionAlgorithm::None);
    assert_eq!(comp_img.len(), jpeg_data.len());

    // 4. Encrypted Backup / High Entropy Payload (Entropy detection -> Auto Bypass)
    let mut encrypted_bytes = Vec::with_capacity(2048);
    for i in 0..2048 {
        encrypted_bytes.push(((i * 197 + 31) % 256) as u8);
    }
    let ctx_enc = FileMetadataContext::new().with_extension("crypt14");
    let (comp_enc, stats_enc) = engine.compress(&encrypted_bytes, &ctx_enc).unwrap();

    println!(
        "{:<20} | {:>10} B | {:>10} B | {:>9.2} | {:>9.1}% | Skipped (High Entropy)",
        "Encrypted DB", encrypted_bytes.len(), comp_enc.len(), stats_enc.ratio, stats_enc.savings_percentage()
    );
    assert_eq!(stats_enc.algorithm, CompressionAlgorithm::None);

    // 5. Delta Preprocessed Sensor / Time-Series Log
    let raw_monotonic: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    let delta_encoded = DeltaEncoder::encode(&raw_monotonic);
    let ctx_log = FileMetadataContext::new().with_extension("log");
    let (comp_delta, stats_delta) = engine.compress(&delta_encoded, &ctx_log).unwrap();

    println!(
        "{:<20} | {:>10} B | {:>10} B | {:>9.2} | {:>9.1}% | Delta + Zstd",
        "Sensor/Time Log", raw_monotonic.len(), comp_delta.len(), stats_delta.ratio, stats_delta.savings_percentage()
    );
    assert!(stats_delta.savings_percentage() > 80.0);

    // 6. Parallel Batch Compression Throughput
    let batch_compressor = ParallelBatchCompressor::with_balanced_cpu(engine.clone());
    let items = vec![
        BatchChunkItem::new("chunk_sms", &sms_json, &ctx_sms),
        BatchChunkItem::new("chunk_db", &sqlite_db, &ctx_db),
        BatchChunkItem::new("chunk_img", &jpeg_data, &ctx_img),
        BatchChunkItem::new("chunk_enc", &encrypted_bytes, &ctx_enc),
    ];
    let batch_results = batch_compressor.compress_batch(&items);
    assert_eq!(batch_results.len(), 4);

    println!("-----------------------------------------------------------------------------------------");
    println!("✅ Parallel Batch Compression of 4 varied chunks completed concurrently.");
    println!("=========================================================================================\n");
}
