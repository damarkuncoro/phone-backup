use phone_backup_chunking::{ChunkConfig, ChunkingMethod, ExpertChunker, TransferMedium};

#[test]
fn test_transfer_medium_configurations() {
    let local = ChunkConfig::for_medium(TransferMedium::HighSpeedLocal);
    assert_eq!(local.min_size, 1024);
    assert_eq!(local.avg_size, 8192);
    assert_eq!(local.max_size, 65536);

    let wireless = ChunkConfig::for_medium(TransferMedium::WirelessAgent);
    assert_eq!(wireless.min_size, 4096);
    assert_eq!(wireless.avg_size, 16384);
    assert_eq!(wireless.max_size, 131072);

    let cloud = ChunkConfig::for_medium(TransferMedium::CloudWebDav);
    assert_eq!(cloud.min_size, 16384);
    assert_eq!(cloud.avg_size, 65536);
    assert_eq!(cloud.max_size, 262144);

    let thermal = ChunkConfig::for_medium(TransferMedium::ThermalConstrained);
    assert_eq!(thermal.min_size, 32768);
    assert_eq!(thermal.avg_size, 131072);
    assert_eq!(thermal.max_size, 524288);
}

#[test]
fn test_adaptive_chunking_execution() {
    // 256 KB of repeating synthetic text
    let mut data = Vec::with_capacity(256 * 1024);
    for i in 0..256 {
        data.extend_from_slice(format!("Payload chunk test line block #{:04}\n", i).as_bytes());
    }

    let local_cfg = ChunkConfig::for_medium(TransferMedium::HighSpeedLocal);
    let local_chunks = ExpertChunker::chunk_data(&data, ChunkingMethod::FastCDC, local_cfg)
        .expect("local chunking failed");
    assert!(!local_chunks.is_empty());

    let cloud_cfg = ChunkConfig::for_medium(TransferMedium::CloudWebDav);
    let cloud_chunks = ExpertChunker::chunk_data(&data, ChunkingMethod::FastCDC, cloud_cfg)
        .expect("cloud chunking failed");
    assert!(!cloud_chunks.is_empty());

    // Local fine-grained chunking should produce >= number of chunks compared to cloud
    assert!(local_chunks.len() >= cloud_chunks.len());
}
