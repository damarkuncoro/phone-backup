/// PerceptualMatcher: Evaluates perceptual distance between image hashes
/// using Hamming Distance and percentage similarity metrics.
pub struct PerceptualMatcher;

impl PerceptualMatcher {
    /// Calculate Hamming distance (number of differing bits) between two 64-bit hashes.
    pub fn hamming_distance(hash_a: u64, hash_b: u64) -> u32 {
        (hash_a ^ hash_b).count_ones()
    }

    /// Calculate visual similarity score between 0.0 (completely distinct) and 1.0 (identical).
    pub fn similarity(hash_a: u64, hash_b: u64) -> f64 {
        let distance = Self::hamming_distance(hash_a, hash_b);
        (64.0 - distance as f64) / 64.0
    }

    /// Check if two images are near-duplicates within a given Hamming distance threshold.
    /// Standard threshold: <= 5 bits difference indicates near-identical visual content.
    pub fn is_near_duplicate(hash_a: u64, hash_b: u64, max_distance_threshold: u32) -> bool {
        Self::hamming_distance(hash_a, hash_b) <= max_distance_threshold
    }
}
