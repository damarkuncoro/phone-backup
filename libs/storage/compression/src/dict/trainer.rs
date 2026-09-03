use anyhow::Result;

/// Trains custom compression dictionaries from data samples.
pub struct DictionaryTrainer;

impl DictionaryTrainer {
    /// Trains a Zstandard dictionary from an array of sample slices.
    pub fn train_from_samples(samples: &[&[u8]], max_dict_size: usize) -> Result<Vec<u8>> {
        let dict = zstd::dict::from_samples(samples, max_dict_size)?;
        Ok(dict)
    }

    /// Creates a simple dictionary from concatenated key string patterns.
    pub fn create_from_patterns(patterns: &[&str]) -> Vec<u8> {
        let mut buffer = Vec::new();
        for p in patterns {
            buffer.extend_from_slice(p.as_bytes());
            buffer.push(b'\n');
        }
        buffer
    }
}
