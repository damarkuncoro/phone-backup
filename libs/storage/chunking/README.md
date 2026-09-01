# phone-backup-chunking 🧩

High-performance, expert-level streaming chunking library for the phone-backup engine. It implements V4.0 specifications for content-defined chunking (CDC) to maximize storage efficiency through deduplication.

## Features

- **Expert Strategy Pattern**: Multiple chunking strategies tailored for different data types.
- **FastCDC (v2020)**: Modern, high-speed content-defined chunking for databases and documents.
- **Fixed-Size Chunking**: Optimal strategy for video and large media files to ensure consistent storage blocks.
- **FullFile Strategy**: Pass-through strategy for small files to avoid unnecessary fragmentation.
- **Streaming Pipeline**: Low memory footprint (bounded channels) regardless of file size.

## Architecture

Part of the `libs/storage` modular layer. It separates the "How to chunk" logic from the storage and application layers.

## Usage

```rust
use chunking::{ExpertChunker, ChunkingMethod, ChunkConfig};

let data = std::fs::read("large_file.db")?;
let config = ChunkConfig::default();

// Get chunks and their data
let chunks = ExpertChunker::chunk_data(&data, ChunkingMethod::FastCDC, config)?;

// Use streaming API for very large files
let mut stream = ExpertChunker::create_stream(reader, ChunkingMethod::FixedSize, config);
while let Some((chunk_info, chunk_data)) = stream.next_chunk()? {
    // Process chunk
}
```
