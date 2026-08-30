/// Trait for reporting progress of long-running operations.
pub trait ProgressPort: Send + Sync {
    /// Initialize the progress tracker with a total count.
    fn start(&self, total: u64, message: &str);

    /// Increment progress by a certain amount.
    fn inc(&self, amount: u64, message: &str);

    /// Mark the operation as finished.
    fn finish(&self, message: &str);

    /// Report an error that occurred during the operation.
    fn error(&self, message: &str);

    /// Log a detailed message without changing progress state.
    fn log(&self, message: &str);
}

/// A no-op implementation of ProgressPort.
pub struct NoProgress;
impl ProgressPort for NoProgress {
    fn start(&self, _total: u64, _message: &str) {}
    fn inc(&self, _amount: u64, _message: &str) {}
    fn finish(&self, _message: &str) {}
    fn error(&self, _message: &str) {}
    fn log(&self, _message: &str) {}
}
