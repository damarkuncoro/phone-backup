use anyhow::Result;
use super::CompressionStrategy;

#[derive(Default)]
pub struct NoCompressionStrategy;

impl CompressionStrategy for NoCompressionStrategy {
    fn name(&self) -> &'static str {
        "none"
    }

    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }
}
