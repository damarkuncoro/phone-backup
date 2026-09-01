# phone-backup-compression 🗜️

Expert compression library for the phone-backup engine. Designed to reduce storage footprint while maintaining high throughput using the latest industry-standard algorithms.

## Features

- **Expert Strategy Pattern**: Supports multiple compression algorithms through a unified interface.
- **Zstd (Zstandard)**: High-speed compression with excellent ratios, ideal for incremental backups.
- **MIME-Aware Policy**: Integrated `should_compress` logic to automatically skip binary media files (images/video) that wouldn't benefit from compression.
- **Expert Strategy**: Allows run-time switching between algorithms based on backup policy.

## Architecture

Part of the `libs/storage` modular layer. This library handles the transformation of data blocks before they are encrypted and stored in the physical layer.

## Usage

```rust
use compression::{ExpertCompressor, CompressionAlgorithm};

let data = b"repeated data... repeated data...";

// High-speed Zstd compression
let compressed = ExpertCompressor::compress(data, CompressionAlgorithm::Zstd)?;

// Smart policy check
if ExpertCompressor::should_compress("application/json") {
    // perform compression
}
```
