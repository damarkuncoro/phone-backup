#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkingMethod {
    /// Simple fixed-size chunks. Fast but doesn't handle insertions/deletions well.
    FixedSize,
    /// Content-Defined Chunking (FastCDC v2020). Best balance of speed and dedup.
    FastCDC,
    /// Treats the entire file as one chunk. No sub-file deduplication.
    FullFile,
}

/// Dynamic target profiles based on physical transport medium or device constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferMedium {
    /// High-speed local disk or USB 3.2 (Fine-grained: 1KB min, 8KB avg, 64KB max)
    HighSpeedLocal,
    /// Wireless Companion Agent Wi-Fi (Balanced: 4KB min, 16KB avg, 128KB max)
    WirelessAgent,
    /// Cloud Storage / WebDAV NAS (Network optimized: 16KB min, 64KB avg, 256KB max)
    CloudWebDav,
    /// Thermal or Battery Constrained (CPU optimized: 32KB min, 128KB avg, 512KB max)
    ThermalConstrained,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkConfig {
    pub min_size: usize,
    pub avg_size: usize,
    pub max_size: usize,
}

impl ChunkConfig {
    pub fn new(min_size: usize, avg_size: usize, max_size: usize) -> Self {
        Self { min_size, avg_size, max_size }
    }

    /// Automatically constructs optimal chunking parameters for a given transfer medium.
    pub fn for_medium(medium: TransferMedium) -> Self {
        match medium {
            TransferMedium::HighSpeedLocal => Self {
                min_size: 1024,
                avg_size: 8 * 1024,
                max_size: 64 * 1024,
            },
            TransferMedium::WirelessAgent => Self {
                min_size: 4 * 1024,
                avg_size: 16 * 1024,
                max_size: 128 * 1024,
            },
            TransferMedium::CloudWebDav => Self {
                min_size: 16 * 1024,
                avg_size: 64 * 1024,
                max_size: 256 * 1024,
            },
            TransferMedium::ThermalConstrained => Self {
                min_size: 32 * 1024,
                avg_size: 128 * 1024,
                max_size: 512 * 1024,
            },
        }
    }
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
