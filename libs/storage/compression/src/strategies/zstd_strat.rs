use super::CompressionStrategy;
use anyhow::Result;
use std::io::{copy, Cursor};

pub struct ZstdStrategy {
    level: i32,
}

impl ZstdStrategy {
    pub fn new(level: i32) -> Self {
        Self { level }
    }
}

impl Default for ZstdStrategy {
    fn default() -> Self {
        Self::new(3)
    }
}

impl CompressionStrategy for ZstdStrategy {
    fn name(&self) -> &'static str {
        "zstd"
    }

    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = zstd::Encoder::new(Vec::new(), self.level)?;
        copy(&mut Cursor::new(data), &mut encoder)?;
        Ok(encoder.finish()?)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = zstd::Decoder::new(Cursor::new(data))?;
        let mut result = Vec::new();
        copy(&mut decoder, &mut result)?;
        Ok(result)
    }
}
