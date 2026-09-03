use crate::model::WaveformPeaks;

pub struct WaveformGenerator;

impl WaveformGenerator {
    pub const DEFAULT_POINTS: usize = 100;

    /// Generates normalized 0..100 amplitude peak array across the audio byte buffer.
    pub fn generate_peaks(bytes: &[u8], points_count: usize) -> WaveformPeaks {
        if bytes.is_empty() || points_count == 0 {
            return WaveformPeaks::new(Vec::new());
        }

        let chunk_size = std::cmp::max(1, bytes.len() / points_count);
        let mut peaks = Vec::with_capacity(points_count);

        let mut max_val: u8 = 1;
        for i in 0..points_count {
            let start = i * chunk_size;
            let end = std::cmp::min(start + chunk_size, bytes.len());
            if start >= bytes.len() {
                peaks.push(0);
                continue;
            }

            let slice = &bytes[start..end];
            // Compute mean absolute deviation from center (128)
            let mut sum_dev: u64 = 0;
            for &b in slice {
                let dev = (b as i16 - 128).unsigned_abs() as u64;
                sum_dev += dev;
            }
            let avg_dev = (sum_dev / slice.len() as u64) as u8;
            if avg_dev > max_val {
                max_val = avg_dev;
            }
            peaks.push(avg_dev);
        }

        // Normalize peaks to 0..100
        for p in &mut peaks {
            *p = ((*p as u32 * 100) / max_val as u32) as u8;
        }

        WaveformPeaks::new(peaks)
    }
}
