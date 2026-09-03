use super::CompressionStrategy;
use crate::dict::models::CompressionDictionary;
use anyhow::Result;
use std::io::{copy, Cursor};

/// Zstandard strategy that utilizes a shared pre-trained dictionary for enhanced ratio.
pub struct ZstdDictionaryStrategy {
    level: i32,
    dictionary: CompressionDictionary,
}

impl ZstdDictionaryStrategy {
    pub fn new(level: i32, dictionary: CompressionDictionary) -> Self {
        Self { level, dictionary }
    }

    pub fn dictionary(&self) -> &CompressionDictionary {
        &self.dictionary
    }
}

impl CompressionStrategy for ZstdDictionaryStrategy {
    fn name(&self) -> &'static str {
        "zstd-dict"
    }

    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder =
            zstd::Encoder::with_dictionary(Vec::new(), self.level, &self.dictionary.data)?;
        copy(&mut Cursor::new(data), &mut encoder)?;
        Ok(encoder.finish()?)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = zstd::Decoder::with_dictionary(Cursor::new(data), &self.dictionary.data)?;
        let mut result = Vec::new();
        copy(&mut decoder, &mut result)?;
        Ok(result)
    }
}
