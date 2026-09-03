use crate::client::AdbClient;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdbWorkerPool {
    client: AdbClient,
    active_count: Arc<AtomicUsize>,
    max_concurrency: usize,
}

impl AdbWorkerPool {
    pub fn new(client: AdbClient, max_concurrency: usize) -> Self {
        let concurrency = max_concurrency.max(1);
        Self {
            client,
            active_count: Arc::new(AtomicUsize::new(0)),
            max_concurrency: concurrency,
        }
    }

    pub fn max_concurrency(&self) -> usize {
        self.max_concurrency
    }

    pub fn active_workers(&self) -> usize {
        self.active_count.load(Ordering::Relaxed)
    }

    pub fn client(&self) -> &AdbClient {
        &self.client
    }

    pub fn try_acquire(&self) -> Option<AdbWorkerGuard> {
        loop {
            let current = self.active_count.load(Ordering::Relaxed);
            if current >= self.max_concurrency {
                return None;
            }
            if self
                .active_count
                .compare_exchange_weak(
                    current,
                    current + 1,
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return Some(AdbWorkerGuard {
                    active_count: self.active_count.clone(),
                });
            }
        }
    }
}

pub struct AdbWorkerGuard {
    active_count: Arc<AtomicUsize>,
}

impl Drop for AdbWorkerGuard {
    fn drop(&mut self) {
        self.active_count.fetch_sub(1, Ordering::SeqCst);
    }
}
