use anyhow::Result;
use ports::StoragePort;
use std::io::{Cursor, Read};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct MockStorage {
    data: Mutex<HashMap<String, Vec<u8>>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl StoragePort for MockStorage {
    fn write(&self, id: &str, data: &mut dyn Read) -> Result<()> {
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer)?;
        self.data.lock().unwrap().insert(id.to_string(), buffer);
        Ok(())
    }

    fn read(&self, id: &str) -> Result<Box<dyn Read>> {
        let guard = self.data.lock().unwrap();
        let content = guard.get(id).ok_or_else(|| anyhow::anyhow!("Object not found"))?;
        Ok(Box::new(Cursor::new(content.clone())))
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
        Ok(100 * 1024 * 1024 * 1024)
    }
}
