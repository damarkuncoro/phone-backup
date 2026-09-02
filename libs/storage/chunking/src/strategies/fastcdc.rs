use super::{Chunk, StreamingChunker};
use crate::config::ChunkConfig;
use anyhow::Result;
use fastcdc::v2020::StreamCDC;
use std::io::Read;

pub struct FastCdcChunker<R: Read> {
    inner: StreamCDC<R>,
    current_offset: u64,
}

impl<R: Read> FastCdcChunker<R> {
    pub fn new(reader: R, config: ChunkConfig) -> Self {
        Self {
            inner: StreamCDC::new(
                reader,
                config.min_size as u32,
                config.avg_size as u32,
                config.max_size as u32,
            ),
            current_offset: 0,
        }
    }
}

impl<R: Read> StreamingChunker for FastCdcChunker<R> {
    fn next_chunk(&mut self) -> Result<Option<(Chunk, Vec<u8>)>> {
        match self.inner.next() {
            Some(Ok(entry)) => {
                let hash = blake3::hash(&entry.data).to_hex().to_string();
                let chunk = Chunk {
                    hash,
                    offset: self.current_offset,
                    length: entry.length as u32,
                };
                self.current_offset += entry.length as u64;
                Ok(Some((chunk, entry.data)))
            }
            Some(Err(e)) => Err(anyhow::anyhow!("FastCDC error: {}", e)),
            None => Ok(None),
        }
    }
}
