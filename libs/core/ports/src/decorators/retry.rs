use crate::StoragePort;
use anyhow::Result;
use std::io::Read;
use std::time::Duration;

/// Storage Decorator that adds automatic retry logic with exponential backoff.
pub struct RetryStorage<S: StoragePort> {
    inner: S,
    max_retries: usize,
    base_backoff: Duration,
}

impl<S: StoragePort> RetryStorage<S> {
    pub fn new(inner: S, max_retries: usize) -> Self {
        Self {
            inner,
            max_retries,
            base_backoff: Duration::from_millis(50),
        }
    }

    pub fn with_backoff(mut self, backoff: Duration) -> Self {
        self.base_backoff = backoff;
        self
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S: StoragePort> StoragePort for RetryStorage<S> {
    fn write(&self, id: &str, data: &mut dyn Read) -> Result<()> {
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer)?;

        let mut attempts = 0;
        loop {
            attempts += 1;
            let mut cursor = std::io::Cursor::new(&buffer);
            match self.inner.write(id, &mut cursor) {
                Ok(()) => return Ok(()),
                Err(e) if attempts <= self.max_retries => {
                    tracing::warn!(
                        "RetryStorage write attempt {}/{} failed for {}: {}. Retrying...",
                        attempts,
                        self.max_retries,
                        id,
                        e
                    );
                    std::thread::sleep(self.base_backoff * attempts as u32);
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn read(&self, id: &str) -> Result<Box<dyn Read>> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            match self.inner.read(id) {
                Ok(reader) => return Ok(reader),
                Err(e) if attempts <= self.max_retries => {
                    tracing::warn!(
                        "RetryStorage read attempt {}/{} failed for {}: {}. Retrying...",
                        attempts,
                        self.max_retries,
                        id,
                        e
                    );
                    std::thread::sleep(self.base_backoff * attempts as u32);
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn exists(&self, id: &str) -> Result<bool> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            match self.inner.exists(id) {
                Ok(res) => return Ok(res),
                Err(_e) if attempts <= self.max_retries => {
                    std::thread::sleep(self.base_backoff * attempts as u32);
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn delete(&self, id: &str) -> Result<()> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            match self.inner.delete(id) {
                Ok(()) => return Ok(()),
                Err(_e) if attempts <= self.max_retries => {
                    std::thread::sleep(self.base_backoff * attempts as u32);
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn list(&self) -> Result<Vec<String>> {
        self.inner.list()
    }

    fn available_space(&self) -> Result<u64> {
        self.inner.available_space()
    }
}
