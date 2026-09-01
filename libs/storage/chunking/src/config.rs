#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkingMethod {
    /// Simple fixed-size chunks. Fast but doesn't handle insertions/deletions well.
    FixedSize,
    /// Content-Defined Chunking (FastCDC v2020). Best balance of speed and dedup.
    FastCDC,
    /// Treats the entire file as one chunk. No sub-file deduplication.
    FullFile,
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkConfig {
    pub min_size: usize,
    pub avg_size: usize,
    pub max_size: usize,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            min_size: 256 * 1024,      // 256 KB
            avg_size: 1024 * 1024,     // 1 MB
            max_size: 2 * 1024 * 1024, // 2 MB
        }
    }
}
