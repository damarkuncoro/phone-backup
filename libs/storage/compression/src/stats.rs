use crate::config::CompressionAlgorithm;

#[cfg(feature = "derive")]
use serde::{Deserialize, Serialize};

/// Telemetry and statistics regarding a compression operation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub struct CompressionStats {
    pub algorithm: CompressionAlgorithm,
    pub original_size: u64,
    pub compressed_size: u64,
    pub ratio: f64,
    pub duration_us: u128,
    pub saved_bytes: u64,
}

impl CompressionStats {
    pub fn new(
        algorithm: CompressionAlgorithm,
        original_size: u64,
        compressed_size: u64,
        duration_us: u128,
    ) -> Self {
        let ratio = if original_size > 0 {
            compressed_size as f64 / original_size as f64
        } else {
            1.0
        };

        let saved_bytes = original_size.saturating_sub(compressed_size);

        Self {
            algorithm,
            original_size,
            compressed_size,
            ratio,
            duration_us,
            saved_bytes,
        }
    }

    pub fn no_compression(size: u64) -> Self {
        Self {
            algorithm: CompressionAlgorithm::None,
            original_size: size,
            compressed_size: size,
            ratio: 1.0,
            duration_us: 0,
            saved_bytes: 0,
        }
    }

    pub fn savings_percentage(&self) -> f64 {
        (1.0 - self.ratio) * 100.0
    }
}
