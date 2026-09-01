use domain::FileEntry;
use super::{ChunkingMethod, ChunkConfig};

/// Determines the best chunking strategy for a given file.
pub trait ChunkingPolicy: Send + Sync {
    fn determine_strategy(&self, file: &FileEntry) -> (ChunkingMethod, ChunkConfig);
}

pub struct DefaultChunkingPolicy;

impl ChunkingPolicy for DefaultChunkingPolicy {
    fn determine_strategy(&self, file: &FileEntry) -> (ChunkingMethod, ChunkConfig) {
        let mime = file.mime_type.to_lowercase();
        let size = file.size_bytes;

        // 1. Small files: Don't bother with sub-file chunking
        if size < 128 * 1024 {
            return (ChunkingMethod::FullFile, ChunkConfig::default());
        }

        // 2. Video files: Fixed-size is often faster and sufficient
        if mime.starts_with("video/") {
            return (
                ChunkingMethod::FixedSize,
                ChunkConfig {
                    min_size: 1024 * 1024,
                    avg_size: 4 * 1024 * 1024, // 4MB chunks for video
                    max_size: 8 * 1024 * 1024,
                },
            );
        }

        // 3. Databases and highly structured binaries: FastCDC is best
        if mime.contains("sqlite") || mime.contains("database") || mime.contains("binary") || file.name.ends_with(".db") {
            return (
                ChunkingMethod::FastCDC,
                ChunkConfig {
                    min_size: 128 * 1024,
                    avg_size: 512 * 1024, // Smaller average for better dedup in DBs
                    max_size: 2 * 1024 * 1024,
                },
            );
        }

        // 4. Default for everything else (Photos, Documents, APKs)
        (ChunkingMethod::FastCDC, ChunkConfig::default())
    }
}
