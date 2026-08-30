use anyhow::Result;
use std::io::copy;
use std::io::Cursor;

pub struct CompressionEngine;

impl CompressionEngine {
    pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = zstd::Encoder::new(Vec::new(), 3)?;
        copy(&mut Cursor::new(data), &mut encoder)?;
        Ok(encoder.finish()?)
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = zstd::Decoder::new(Cursor::new(data))?;
        let mut result = Vec::new();
        copy(&mut decoder, &mut result)?;
        Ok(result)
    }

    pub fn should_compress(mime_type: &str) -> bool {
        match mime_type {
            "text/plain" | "application/json" | "application/xml" | "text/csv" => true,
            m if m.starts_with("text/") => true,
            _ => false,
        }
    }
}
