use std::time::Instant;

/// Value object calculating transfer rate (bytes/sec, MB/s) and Estimated Time Remaining (ETA).
#[derive(Debug, Clone)]
pub struct ProgressEstimator {
    start_time: Instant,
    total_bytes: u64,
    processed_bytes: u64,
}

impl ProgressEstimator {
    pub fn new(total_bytes: u64) -> Self {
        Self {
            start_time: Instant::now(),
            total_bytes,
            processed_bytes: 0,
        }
    }

    pub fn update(&mut self, add_bytes: u64) {
        self.processed_bytes += add_bytes;
    }

    pub fn set_processed_bytes(&mut self, bytes: u64) {
        self.processed_bytes = bytes;
    }

    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub fn processed_bytes(&self) -> u64 {
        self.processed_bytes
    }

    /// Calculate throughput in MegaBytes per second (MB/s).
    pub fn megabytes_per_second(&self) -> f64 {
        let elapsed_secs = self.start_time.elapsed().as_secs_f64();
        if elapsed_secs <= 0.001 {
            0.0
        } else {
            (self.processed_bytes as f64 / (1024.0 * 1024.0)) / elapsed_secs
        }
    }

    /// Calculate Estimated Time Remaining (ETA) in seconds.
    pub fn eta_seconds(&self) -> Option<u64> {
        let elapsed_secs = self.start_time.elapsed().as_secs_f64();
        if elapsed_secs <= 0.001 || self.processed_bytes == 0 || self.processed_bytes >= self.total_bytes {
            None
        } else {
            let bytes_per_sec = self.processed_bytes as f64 / elapsed_secs;
            let remaining_bytes = self.total_bytes - self.processed_bytes;
            Some((remaining_bytes as f64 / bytes_per_sec).ceil() as u64)
        }
    }

    /// Format ETA as human readable string (e.g. "02m 15s", "45s", or "Completed").
    pub fn format_eta(&self) -> String {
        if self.processed_bytes >= self.total_bytes && self.total_bytes > 0 {
            return "Completed".to_string();
        }

        match self.eta_seconds() {
            Some(secs) => {
                let mins = secs / 60;
                let s = secs % 60;
                if mins > 0 {
                    format!("{:02}m {:02}s", mins, s)
                } else {
                    format!("{}s", s)
                }
            }
            None => "Calculating...".to_string(),
        }
    }
}
