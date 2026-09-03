pub mod analysis;
pub mod config;
pub mod dict;
pub mod engine;
pub mod parallel;
pub mod preprocessing;
pub mod stats;
pub mod strategies;

use anyhow::Result;

pub use analysis::{ContentClassifier, DataCategory, EntropyDetector, SampleProbe};
pub use config::{
    CompressionAlgorithm, CompressionDecision, CompressionLevel, CompressionPolicy,
    FileMetadataContext,
};
pub use dict::{CompressionDictionary, DictionaryId, DictionaryManager, DictionaryTrainer};
pub use engine::{CompressionEngineBuilder, SmartCompressionEngine};
pub use parallel::{BatchChunkItem, BatchCompressedChunk, ParallelBatchCompressor};
pub use preprocessing::DeltaEncoder;
pub use stats::CompressionStats;
pub use strategies::{CompressionStrategy, CompressionStrategyFactory};

/// Backward-compatible facade for existing compression callers.
pub struct ExpertCompressor;

impl ExpertCompressor {
    pub fn get_strategy(algo: CompressionAlgorithm) -> Box<dyn CompressionStrategy> {
        CompressionStrategyFactory::create_default(algo)
    }

    pub fn compress(data: &[u8], algo: CompressionAlgorithm) -> Result<Vec<u8>> {
        Self::get_strategy(algo).compress(data)
    }

    pub fn decompress(data: &[u8], algo: CompressionAlgorithm) -> Result<Vec<u8>> {
        Self::get_strategy(algo).decompress(data)
    }

    pub fn should_compress(mime_type: &str) -> bool {
        let category = ContentClassifier::classify_mime(mime_type);
        !ContentClassifier::is_precompressed(category)
    }
}
