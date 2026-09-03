use super::none::NoCompressionStrategy;
use super::zstd_dict_strat::ZstdDictionaryStrategy;
use super::zstd_strat::ZstdStrategy;
use super::CompressionStrategy;
use crate::config::{CompressionAlgorithm, CompressionLevel};
use crate::dict::models::CompressionDictionary;

/// Factory responsible for instantiating compression strategies.
pub struct CompressionStrategyFactory;

impl CompressionStrategyFactory {
    /// Creates a boxed compression strategy based on algorithm and level.
    pub fn create(
        algorithm: CompressionAlgorithm,
        level: CompressionLevel,
    ) -> Box<dyn CompressionStrategy> {
        match algorithm {
            CompressionAlgorithm::None => Box::new(NoCompressionStrategy),
            CompressionAlgorithm::Zstd => Box::new(ZstdStrategy::new(level.to_zstd_level())),
        }
    }

    /// Creates a boxed compression strategy that utilizes a shared dictionary.
    pub fn create_with_dictionary(
        algorithm: CompressionAlgorithm,
        level: CompressionLevel,
        dictionary: CompressionDictionary,
    ) -> Box<dyn CompressionStrategy> {
        match algorithm {
            CompressionAlgorithm::None => Box::new(NoCompressionStrategy),
            CompressionAlgorithm::Zstd => Box::new(ZstdDictionaryStrategy::new(
                level.to_zstd_level(),
                dictionary,
            )),
        }
    }

    /// Creates a default strategy for the given algorithm.
    pub fn create_default(algorithm: CompressionAlgorithm) -> Box<dyn CompressionStrategy> {
        Self::create(algorithm, CompressionLevel::default())
    }
}
