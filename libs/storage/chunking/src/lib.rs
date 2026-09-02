use anyhow::Result;
use std::io::Read;

pub mod config;
pub mod strategies;

pub use config::{ChunkConfig, ChunkingMethod};
pub use strategies::{Chunk, Chunker, StreamingChunker};

/// The Expert Chunker Factory
pub struct ExpertChunker;

impl ExpertChunker {
    /// Creates a streaming chunker based on the selected method.
    pub fn create_stream<R: Read + 'static>(
        reader: R,
        method: ChunkingMethod,
        config: ChunkConfig,
    ) -> Box<dyn StreamingChunker> {
        match method {
            ChunkingMethod::FixedSize => {
                Box::new(strategies::fixed::FixedChunker::new(reader, config))
            }
            ChunkingMethod::FastCDC => {
                Box::new(strategies::fastcdc::FastCdcChunker::new(reader, config))
            }
            ChunkingMethod::FullFile => {
                Box::new(strategies::fullfile::FullFileChunker::new(reader, config))
            }
        }
    }

    /// Helper for quick static data chunking.
    pub fn chunk_data(
        data: &[u8],
        method: ChunkingMethod,
        config: ChunkConfig,
    ) -> Result<Vec<(Chunk, Vec<u8>)>> {
        let mut results = Vec::new();
        let stream_reader = std::io::Cursor::new(data.to_vec());
        let mut stream = Self::create_stream(stream_reader, method, config);

        while let Some(chunk_result) = stream.next_chunk()? {
            results.push(chunk_result);
        }

        Ok(results)
    }
}
