use crate::strategies::zstd_strat::ZstdStrategy;
use crate::strategies::CompressionStrategy;

/// Quick compression compressibility sampler.
pub struct SampleProbe;

impl SampleProbe {
    pub const DEFAULT_SAMPLE_SIZE: usize = 64 * 1024; // 64 KB

    /// Probes a slice of data with fast compression to measure compressibility ratio.
    /// Returns Some(ratio) if probe succeeded, where ratio = compressed_len / original_len.
    pub fn probe_compressibility(data: &[u8], sample_size: usize) -> Option<f64> {
        if data.is_empty() {
            return None;
        }

        let sample_len = data.len().min(sample_size);
        if sample_len < 128 {
            return None;
        }

        let sample = &data[..sample_len];
        let fast_strategy = ZstdStrategy::new(1); // fast level 1 for probing

        match fast_strategy.compress(sample) {
            Ok(compressed) => {
                let ratio = compressed.len() as f64 / sample_len as f64;
                Some(ratio)
            }
            Err(_) => None,
        }
    }

    /// Evaluates whether sample compression achieves at least minimum space reduction.
    pub fn is_worth_compressing(data: &[u8], sample_size: usize, min_saving_ratio: f64) -> bool {
        match Self::probe_compressibility(data, sample_size) {
            Some(ratio) => ratio <= (1.0 - min_saving_ratio),
            None => true,
        }
    }
}
