use std::io::Cursor;
use phone_backup_application::storage::{Chunker, ChunkConfig, ChunkingMethod};

#[test]
fn test_streaming_chunker_basic() {
    // 1. Setup: 10MB of repetitive but identifiable data
    let mut data = Vec::with_capacity(10 * 1024 * 1024);
    for i in 0..10 * 1024 * 1024 {
        data.push((i % 256) as u8);
    }
    let reader = Cursor::new(data);

    // 2. Config: Average 1MB, Min 256KB, Max 2MB
    let config = ChunkConfig {
        min_size: 256 * 1024,
        avg_size: 1024 * 1024,
        max_size: 2048 * 1024,
    };

    // Use Expert Chunker Factory (re-exported as Chunker)
    let mut chunker = Chunker::create_stream(reader, ChunkingMethod::FastCDC, config);

    let mut total_size = 0;
    let mut chunk_count = 0;

    // 3. Action: Process chunks
    while let Some(result) = chunker.next_chunk().unwrap() {
        let (chunk_metadata, chunk_data) = result;

        assert_eq!(chunk_metadata.length as usize, chunk_data.len());
        // Last chunk can be smaller than min_size

        total_size += chunk_metadata.length as u64;
        chunk_count += 1;
    }

    // 4. Assert
    assert_eq!(total_size, 10 * 1024 * 1024);
    assert!(chunk_count > 1);
    println!("Total chunks: {}", chunk_count);
}

#[test]
fn test_fixed_size_chunking() {
    let data = vec![0u8; 1024 * 1024]; // 1MB
    let config = ChunkConfig {
        min_size: 256 * 1024,
        avg_size: 256 * 1024, // 256KB fixed
        max_size: 256 * 1024,
    };

    let results = Chunker::chunk_data(&data, ChunkingMethod::FixedSize, config).unwrap();

    // 1MB / 256KB = 4 chunks
    assert_eq!(results.len(), 4);
    for (metadata, chunk_data) in results {
        assert_eq!(metadata.length, 256 * 1024);
        assert_eq!(chunk_data.len(), 256 * 1024);
    }
}
