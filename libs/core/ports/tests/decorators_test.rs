use anyhow::Result;
use phone_backup_ports::decorators::{MetricsStorage, RetryStorage};
use phone_backup_ports::StoragePort;
use std::io::Read;
use std::sync::atomic::Ordering;
use std::time::Duration;

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
