use domain::ScanMetrics;
use std::time::Instant;

/// Performance tracker for device scan operations.
pub struct ScanMetricsTracker {
    start_time: Instant,
    directories_scanned: usize,
    files_scanned: usize,
}

impl ScanMetricsTracker {
    pub fn start() -> Self {
        Self {
            start_time: Instant::now(),
            directories_scanned: 0,
            files_scanned: 0,
        }
    }

    pub fn add_directories(&mut self, count: usize) {
        self.directories_scanned += count;
    }

    pub fn set_files_scanned(&mut self, count: usize) {
        self.files_scanned = count;
    }

    pub fn finish(&self) -> ScanMetrics {
        let elapsed = self.start_time.elapsed();
        let duration_ms = elapsed.as_millis() as u64;
        let secs = elapsed.as_secs_f64();
        let throughput_files_per_sec = if secs > 0.0001 {
            self.files_scanned as f64 / secs
        } else {
            0.0
        };

        ScanMetrics {
            duration_ms,
            directories_scanned: self.directories_scanned,
            files_scanned: self.files_scanned,
            throughput_files_per_sec,
        }
    }
}
