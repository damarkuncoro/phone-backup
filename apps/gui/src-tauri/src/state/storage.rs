use std::sync::{Arc, RwLock};
use ports::StoragePort;
use std::io::Read;

/// Modular Storage Switcher (SOLID - LSP)
pub struct SwitchableStorage {
    current: RwLock<Box<dyn StoragePort>>,
}

impl SwitchableStorage {
    pub fn new(initial: Box<dyn StoragePort>) -> Self {
        Self {
            current: RwLock::new(initial),
        }
    }

    pub fn switch(&self, new_storage: Box<dyn StoragePort>) {
        let mut writer = self.current.write().unwrap();
        *writer = new_storage;
    }
}

impl StoragePort for SwitchableStorage {
    fn write(&self, id: &str, data: &mut dyn Read) -> anyhow::Result<()> {
        self.current.read().unwrap().write(id, data)
    }
    fn read(&self, id: &str) -> anyhow::Result<Box<dyn Read>> {
        self.current.read().unwrap().read(id)
    }
    fn exists(&self, id: &str) -> anyhow::Result<bool> {
        self.current.read().unwrap().exists(id)
    }
    fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.current.read().unwrap().delete(id)
    }
    fn list(&self) -> anyhow::Result<Vec<String>> {
        self.current.read().unwrap().list()
    }
    fn available_space(&self) -> anyhow::Result<u64> {
        self.current.read().unwrap().available_space()
    }
}

/// Newtype wrapper to satisfy orphan rules for Arc + Trait
#[derive(Clone)]
pub struct SharedStorage(pub Arc<SwitchableStorage>);

impl StoragePort for SharedStorage {
    fn write(&self, id: &str, data: &mut dyn Read) -> anyhow::Result<()> {
        self.0.write(id, data)
    }
    fn read(&self, id: &str) -> anyhow::Result<Box<dyn Read>> {
        self.0.read(id)
    }
    fn exists(&self, id: &str) -> anyhow::Result<bool> {
        self.0.exists(id)
    }
    fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.0.delete(id)
    }
    fn list(&self) -> anyhow::Result<Vec<String>> {
        self.0.list()
    }
    fn available_space(&self) -> anyhow::Result<u64> {
        self.0.available_space()
    }
}
