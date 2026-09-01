use anyhow::Result;
use std::io::Read;

/// Port for the physical storage of backup objects.
pub trait StoragePort: Send + Sync {
    /// Write a blob of data to storage and return its identifier (e.g., path or hash).
    fn write(&self, id: &str, data: &mut dyn Read) -> Result<()>;

    /// Read a blob of data from storage.
    fn read(&self, id: &str) -> Result<Box<dyn Read>>;

    /// Check if a blob exists.
    fn exists(&self, id: &str) -> Result<bool>;

    /// Delete a blob.
    fn delete(&self, id: &str) -> Result<()>;

    /// List all blob IDs in the storage.
    fn list(&self) -> Result<Vec<String>>;

    /// Return available free space in bytes on target storage.
    fn available_space(&self) -> Result<u64>;
}

impl<S: StoragePort + ?Sized> StoragePort for Box<S> {
    fn write(&self, id: &str, data: &mut dyn Read) -> Result<()> {
        (**self).write(id, data)
    }

    fn read(&self, id: &str) -> Result<Box<dyn Read>> {
        (**self).read(id)
    }

    fn exists(&self, id: &str) -> Result<bool> {
        (**self).exists(id)
    }

    fn delete(&self, id: &str) -> Result<()> {
        (**self).delete(id)
    }

    fn list(&self) -> Result<Vec<String>> {
        (**self).list()
    }

    fn available_space(&self) -> Result<u64> {
        (**self).available_space()
    }
}
