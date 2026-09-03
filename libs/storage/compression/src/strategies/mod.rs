use anyhow::Result;

pub mod factory;
pub mod none;
pub mod zstd_dict_strat;
pub mod zstd_strat;

pub use factory::CompressionStrategyFactory;
pub use none::NoCompressionStrategy;
pub use zstd_dict_strat::ZstdDictionaryStrategy;
pub use zstd_strat::ZstdStrategy;

/// Abstraction for pluggable compression strategy algorithms.
pub trait CompressionStrategy: Send + Sync {
    /// Identifier name of the strategy (e.g. "none", "zstd", "zstd-dict").
    fn name(&self) -> &'static str;

    /// Compresses the provided raw data slice into output bytes.
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>>;

    /// Decompresses the input bytes back to original raw data.
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
}
