use super::{Chunk, StreamingChunker};
use crate::config::ChunkConfig;
use anyhow::Result;
use std::io::Read;

pub struct FixedChunker<R: Read> {
    reader: R,
    chunk_size: usize,
    current_offset: u64,
}

impl<R: Read> FixedChunker<R> {
    pub fn new(reader: R, config: ChunkConfig) -> Self {
        Self {
            reader,
            chunk_size: config.avg_size,
            current_offset: 0,
        }
    }
}

impl<R: Read> StreamingChunker for FixedChunker<R> {
    fn next_chunk(&mut self) -> Result<Option<(Chunk, Vec<u8>)>> {
        let mut buffer = vec![0u8; self.chunk_size];
        let bytes_read = self.reader.read(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(None);
        }

        buffer.truncate(bytes_read);
        let hash = blake3::hash(&buffer).to_hex().to_string();

        let chunk = Chunk {
            hash,
            offset: self.current_offset,
            length: bytes_read as u32,
        };

        self.current_offset += bytes_read as u64;
        Ok(Some((chunk, buffer)))
    }
}
