use crate::StoragePort;
use anyhow::Result;
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
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
        // Read data into buffer so we can re-read it across retries if needed
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

/// Storage metrics snapshot.
#[derive(Debug, Clone, Default)]
pub struct StorageMetrics {
    pub bytes_written: u64,
    pub bytes_read: u64,
    pub write_ops: u64,
    pub read_ops: u64,
    pub delete_ops: u64,
}

/// Storage Decorator that records transfer throughput, operations, and telemetry.
pub struct MetricsStorage<S: StoragePort> {
    inner: S,
    bytes_written: Arc<AtomicU64>,
    bytes_read: Arc<AtomicU64>,
    write_ops: Arc<AtomicU64>,
    read_ops: Arc<AtomicU64>,
    delete_ops: Arc<AtomicU64>,
}

impl<S: StoragePort> MetricsStorage<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            bytes_written: Arc::new(AtomicU64::new(0)),
            bytes_read: Arc::new(AtomicU64::new(0)),
            write_ops: Arc::new(AtomicU64::new(0)),
            read_ops: Arc::new(AtomicU64::new(0)),
            delete_ops: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn metrics(&self) -> StorageMetrics {
        StorageMetrics {
            bytes_written: self.bytes_written.load(Ordering::Relaxed),
            bytes_read: self.bytes_read.load(Ordering::Relaxed),
            write_ops: self.write_ops.load(Ordering::Relaxed),
            read_ops: self.read_ops.load(Ordering::Relaxed),
            delete_ops: self.delete_ops.load(Ordering::Relaxed),
        }
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S: StoragePort> StoragePort for MetricsStorage<S> {
    fn write(&self, id: &str, data: &mut dyn Read) -> Result<()> {
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer)?;
        let len = buffer.len() as u64;

        let mut cursor = std::io::Cursor::new(buffer);
        self.inner.write(id, &mut cursor)?;

        self.bytes_written.fetch_add(len, Ordering::Relaxed);
        self.write_ops.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn read(&self, id: &str) -> Result<Box<dyn Read>> {
        let mut reader = self.inner.read(id)?;
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let len = buffer.len() as u64;

        self.bytes_read.fetch_add(len, Ordering::Relaxed);
        self.read_ops.fetch_add(1, Ordering::Relaxed);

        Ok(Box::new(std::io::Cursor::new(buffer)))
    }

    fn exists(&self, id: &str) -> Result<bool> {
        self.inner.exists(id)
    }

    fn delete(&self, id: &str) -> Result<()> {
        self.inner.delete(id)?;
        self.delete_ops.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn list(&self) -> Result<Vec<String>> {
        self.inner.list()
    }

    fn available_space(&self) -> Result<u64> {
        self.inner.available_space()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStorage {
        data: std::sync::Mutex<std::collections::HashMap<String, Vec<u8>>>,
        fail_times: std::sync::atomic::AtomicUsize,
    }

    impl MockStorage {
        fn new(fail_times: usize) -> Self {
            Self {
                data: std::sync::Mutex::new(std::collections::HashMap::new()),
                fail_times: std::sync::atomic::AtomicUsize::new(fail_times),
            }
        }
    }

    impl StoragePort for MockStorage {
        fn write(&self, id: &str, data: &mut dyn Read) -> Result<()> {
            let prev = self.fail_times.load(Ordering::SeqCst);
            if prev > 0 {
                self.fail_times.store(prev - 1, Ordering::SeqCst);
                anyhow::bail!("Simulated transient error");
            }
            let mut buf = Vec::new();
            data.read_to_end(&mut buf)?;
            self.data.lock().unwrap().insert(id.to_string(), buf);
            Ok(())
        }

        fn read(&self, id: &str) -> Result<Box<dyn Read>> {
            let data = self
                .data
                .lock()
                .unwrap()
                .get(id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Not found"))?;
            Ok(Box::new(std::io::Cursor::new(data)))
        }

        fn exists(&self, id: &str) -> Result<bool> {
            Ok(self.data.lock().unwrap().contains_key(id))
        }

        fn delete(&self, id: &str) -> Result<()> {
            self.data.lock().unwrap().remove(id);
            Ok(())
        }

        fn list(&self) -> Result<Vec<String>> {
            Ok(self.data.lock().unwrap().keys().cloned().collect())
        }

        fn available_space(&self) -> Result<u64> {
            Ok(1024 * 1024 * 1024)
        }
    }

    #[test]
    fn test_retry_storage_decorator() {
        // Storage fails twice, succeeds on 3rd try
        let raw = MockStorage::new(2);
        let storage = RetryStorage::new(raw, 3).with_backoff(Duration::from_millis(1));

        let mut data: &[u8] = b"hello world";
        assert!(storage.write("test_key", &mut data).is_ok());
        assert!(storage.exists("test_key").unwrap());
    }

    #[test]
    fn test_metrics_storage_decorator() {
        let raw = MockStorage::new(0);
        let storage = MetricsStorage::new(raw);

        let mut data: &[u8] = b"12345";
        storage.write("metric_test", &mut data).unwrap();

        let mut reader = storage.read("metric_test").unwrap();
        let mut out = String::new();
        reader.read_to_string(&mut out).unwrap();

        let metrics = storage.metrics();
        assert_eq!(metrics.bytes_written, 5);
        assert_eq!(metrics.bytes_read, 5);
        assert_eq!(metrics.write_ops, 1);
        assert_eq!(metrics.read_ops, 1);
    }
}
