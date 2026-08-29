use anyhow::Result;
use ports::StoragePort;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

pub struct LocalStorage {
    base_dir: PathBuf,
}

impl LocalStorage {
    pub fn new(base_dir: impl Into<PathBuf>) -> Result<Self> {
        let base_dir = base_dir.into();
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }
        Ok(Self { base_dir })
    }
}

impl StoragePort for LocalStorage {
    fn write(&self, id: &str, data: &mut dyn Read) -> Result<()> {
        let path = self.base_dir.join(id);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        io::copy(data, &mut file)?;
        Ok(())
    }

    fn read(&self, id: &str) -> Result<Box<dyn Read>> {
        let path = self.base_dir.join(id);
        let file = File::open(path)?;
        Ok(Box::new(file))
    }

    fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.base_dir.join(id).exists())
    }

    fn delete(&self, id: &str) -> Result<()> {
        let path = self.base_dir.join(id);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    fn list(&self) -> Result<Vec<String>> {
        use walkdir::WalkDir;
        let mut results = Vec::new();
        for entry in WalkDir::new(&self.base_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let rel_path = entry.path().strip_prefix(&self.base_dir)?;
                results.push(rel_path.to_string_lossy().into_owned());
            }
        }
        Ok(results)
    }
}
