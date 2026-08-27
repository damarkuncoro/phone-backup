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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_decompression_roundtrip() {
        let data = b"data yang berulang-ulang ulang-ulang ulang-ulang";
        let compressed = CompressionEngine::compress(data).unwrap();
        assert!(compressed.len() > 0);

        let decompressed = CompressionEngine::decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_should_compress_policy() {
        assert!(CompressionEngine::should_compress("text/plain"));
        assert!(CompressionEngine::should_compress("application/json"));
        assert!(!CompressionEngine::should_compress("image/jpeg"));
        assert!(!CompressionEngine::should_compress("video/mp4"));
    }
}
