use crate::StoragePort;
use anyhow::Result;
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

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
