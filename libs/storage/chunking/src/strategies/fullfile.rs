use super::{Chunk, StreamingChunker};
use crate::config::ChunkConfig;
use anyhow::Result;
use std::io::Read;

/// A pseudo-chunker that treats the entire stream as one single chunk.
/// Useful for small files or when chunking is disabled.
pub struct FullFileChunker<R: Read> {
    reader: Option<R>,
    current_offset: u64,
}

impl<R: Read> FullFileChunker<R> {
    pub fn new(reader: R, _config: ChunkConfig) -> Self {
        Self {
            reader: Some(reader),
            current_offset: 0,
        }
    }
}

impl<R: Read> StreamingChunker for FullFileChunker<R> {
    fn next_chunk(&mut self) -> Result<Option<(Chunk, Vec<u8>)>> {
        if let Some(mut reader) = self.reader.take() {
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;

            if buffer.is_empty() {
                return Ok(None);
            }

            let hash = blake3::hash(&buffer).to_hex().to_string();
            let length = buffer.len() as u32;

            let chunk = Chunk {
                hash,
                offset: self.current_offset,
                length,
            };

            self.current_offset += length as u64;
            Ok(Some((chunk, buffer)))
        } else {
            Ok(None)
        }
    }
}
