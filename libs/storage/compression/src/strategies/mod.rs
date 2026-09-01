use anyhow::Result;

pub mod zstd_strat;
pub mod none;

pub trait CompressionStrategy: Send + Sync {
    fn name(&self) -> &'static str;
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
}
