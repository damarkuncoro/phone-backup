use anyhow::Result;

pub mod fixed;
pub mod fastcdc;
pub mod fullfile;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub hash: String,
    pub offset: u64,
    pub length: u32,
}

/// A trait for streaming chunking implementations
pub trait StreamingChunker {
    /// Returns the next chunk and its data, or None if end of stream.
    fn next_chunk(&mut self) -> Result<Option<(Chunk, Vec<u8>)>>;
}

/// Standard trait for all chunker logic
pub trait Chunker {
    fn name(&self) -> &'static str;
}
