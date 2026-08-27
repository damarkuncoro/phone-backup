/// Observer interface for tracking progress of long-running operations.
pub trait ProgressObserver: Send + Sync {
    fn start(&self, total_items: u64, message: &str);
    fn update(&self, current: u64, message: &str);
    fn finish(&self, message: &str);
}

/// Default no-op implementation used when no progress tracking is desired (e.g. tests or headless).
#[derive(Default, Debug, Clone, Copy)]
pub struct NoopProgressObserver;

impl ProgressObserver for NoopProgressObserver {
    fn start(&self, _total_items: u64, _message: &str) {}
    fn update(&self, _current: u64, _message: &str) {}
    fn finish(&self, _message: &str) {}
}
