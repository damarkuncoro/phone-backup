use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// A thread-safe token used to cooperatively signal cancellation across threads and tasks.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Request cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Check whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// Reset the cancellation token to active (not cancelled).
    pub fn reset(&self) {
        self.cancelled.store(false, Ordering::SeqCst);
    }

    /// Returns an error if cancellation was requested.
    pub fn check_cancelled(&self) -> Result<(), String> {
        if self.is_cancelled() {
            Err("Operation was cancelled by user".to_string())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());
        assert!(token.check_cancelled().is_ok());

        let token_clone = token.clone();
        token_clone.cancel();

        assert!(token.is_cancelled());
        assert!(token.check_cancelled().is_err());

        token.reset();
        assert!(!token.is_cancelled());
    }
}
