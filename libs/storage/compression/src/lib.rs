pub mod strategies;
pub mod config;

use anyhow::Result;
pub use config::CompressionAlgorithm;
pub use strategies::CompressionStrategy;

pub struct ExpertCompressor;

impl ExpertCompressor {
    pub fn get_strategy(algo: CompressionAlgorithm) -> Box<dyn CompressionStrategy> {
        match algo {
            CompressionAlgorithm::None => Box::new(strategies::none::NoCompressionStrategy::default()),
            CompressionAlgorithm::Zstd => Box::new(strategies::zstd_strat::ZstdStrategy::default()),
        }
    }

    pub fn compress(data: &[u8], algo: CompressionAlgorithm) -> Result<Vec<u8>> {
        Self::get_strategy(algo).compress(data)
    }

    pub fn decompress(data: &[u8], algo: CompressionAlgorithm) -> Result<Vec<u8>> {
        Self::get_strategy(algo).decompress(data)
    }

    /// Determines if a file should be compressed based on its MIME type.
    pub fn should_compress(mime_type: &str) -> bool {
        match mime_type {
            "text/plain" | "application/json" | "application/xml" | "text/csv" => true,
            m if m.starts_with("text/") => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zstd_roundtrip() {
        let data = b"expert compression test data expert compression test data";
        let compressed = ExpertCompressor::compress(data, CompressionAlgorithm::Zstd).unwrap();
        assert!(compressed.len() < data.len());

        let decompressed = ExpertCompressor::decompress(&compressed, CompressionAlgorithm::Zstd).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_none_roundtrip() {
        let data = b"some data";
        let compressed = ExpertCompressor::compress(data, CompressionAlgorithm::None).unwrap();
        assert_eq!(data.to_vec(), compressed);
    }
}
