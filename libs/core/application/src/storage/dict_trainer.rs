use super::manager::ObjectManager;
use anyhow::Result;
use compression::{CompressionDictionary, DataCategory, DictionaryTrainer};
use domain::EncryptionMode;
use ports::{RepositoryPort, StoragePort};
use std::io::Cursor;
use tracing::info;

/// Background service for training customized compression dictionaries from backed-up data.
pub struct AutoDictionaryService<'a, T: StoragePort, R: RepositoryPort> {
    storage: &'a T,
    repository: &'a R,
    encryption: &'a EncryptionMode,
}

impl<'a, T: StoragePort, R: RepositoryPort> AutoDictionaryService<'a, T, R> {
    pub fn new(storage: &'a T, repository: &'a R, encryption: &'a EncryptionMode) -> Self {
        Self {
            storage,
            repository,
            encryption,
        }
    }

    /// Gathers sample data from an array of chunk IDs and trains a customized Zstandard dictionary.
    pub fn train_custom_dictionary(
        &self,
        dict_id: &str,
        category: DataCategory,
        sample_chunk_ids: &[String],
        max_dict_size: usize,
    ) -> Result<CompressionDictionary> {
        let object_manager = ObjectManager::new(self.storage, self.repository, self.encryption);
        let mut sample_buffers = Vec::new();

        for chunk_id in sample_chunk_ids {
            if let Ok(data) = object_manager.get_chunk(chunk_id) {
                if !data.is_empty() {
                    sample_buffers.push(data);
                }
            }
        }

        if sample_buffers.is_empty() {
            anyhow::bail!("No valid sample chunks found for dictionary training");
        }

        let sample_slices: Vec<&[u8]> = sample_buffers.iter().map(|b| b.as_slice()).collect();
        let trained_bytes = DictionaryTrainer::train_from_samples(&sample_slices, max_dict_size)?;

        info!(
            "Trained custom dictionary '{}' ({} bytes) from {} samples",
            dict_id,
            trained_bytes.len(),
            sample_slices.len()
        );

        let dict = CompressionDictionary::new(dict_id, category, trained_bytes.clone());

        // Persist dictionary to storage
        let storage_path = format!("dictionaries/{}.dict", dict_id);
        self.storage
            .write(&storage_path, &mut Cursor::new(trained_bytes))?;

        Ok(dict)
    }

    /// Loads a stored dictionary from the storage layer.
    pub fn load_dictionary(&self, dict_id: &str, category: DataCategory) -> Result<CompressionDictionary> {
        let storage_path = format!("dictionaries/{}.dict", dict_id);
        let mut reader = self.storage.read(&storage_path)?;
        let mut data = Vec::new();
        use std::io::Read;
        reader.read_to_end(&mut data)?;

        Ok(CompressionDictionary::new(dict_id, category, data))
    }
}
