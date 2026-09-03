/// Shannon entropy analyzer for detecting randomness or prior compression.
pub struct EntropyDetector;

impl EntropyDetector {
    /// Default entropy threshold above which data is considered incompressible (bits per byte).
    pub const DEFAULT_HIGH_ENTROPY_THRESHOLD: f64 = 7.5;

    /// Calculates Shannon entropy of the byte slice in bits per byte (0.0 to 8.0).
    pub fn calculate_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut frequencies = [0usize; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }

        let total_bytes = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &frequencies {
            if count > 0 {
                let p = count as f64 / total_bytes;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    /// Tests if a sample of the data exceeds the high entropy threshold.
    pub fn is_high_entropy(data: &[u8], threshold: f64) -> bool {
        Self::calculate_entropy(data) >= threshold
    }

    /// Evaluates entropy on a bounded prefix sample (e.g. first 64KB) for performance.
    pub fn sample_entropy(data: &[u8], sample_size: usize) -> f64 {
        let sample = if data.len() > sample_size {
            &data[..sample_size]
        } else {
            data
        };
        Self::calculate_entropy(sample)
    }
}
