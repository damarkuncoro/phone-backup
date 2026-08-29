use anyhow::Result;
use domain::EncryptionMode;
use ports::StoragePort;
use crate::chunking::{Chunker, Chunk};
use crate::compression::CompressionEngine;
use crate::security::EncryptionEngine;
use crate::object_store::ObjectStoreKey;
use crate::hashing::calculate_hash;

pub struct ObjectManager<'a, T: StoragePort> {
    storage: &'a T,
    encryption: &'a EncryptionMode,
}

impl<'a, T: StoragePort> ObjectManager<'a, T> {
    pub fn new(storage: &'a T, encryption: &'a EncryptionMode) -> Self {
        Self { storage, encryption }
    }

    /// Processes a block of data: Hash, (Compress), Encrypt, and Write to storage if not exists.
    /// Returns (hash, size_in_storage, was_compressed)
    pub fn put_object(&self, data: &[u8], mime_type: Option<&str>) -> Result<(String, u64, bool)> {
        let hash = calculate_hash(data);
        let is_encrypted = self.encryption.is_encrypted();

        let mut should_compress = false;
        if let Some(mime) = mime_type {
            should_compress = CompressionEngine::should_compress(mime);
        }

        let object_id = ObjectStoreKey::compute_object_id(&hash, mime_type, is_encrypted);
        let object_path = ObjectStoreKey::compute_object_path(&hash, &object_id);

        if !self.storage.exists(&object_path)? {
            let mut processed_data = data.to_vec();

            if should_compress {
                processed_data = CompressionEngine::compress(&processed_data)?;
            }

            processed_data = match self.encryption {
                EncryptionMode::Password(pwd) => EncryptionEngine::encrypt(&processed_data, pwd)?,
                EncryptionMode::PublicKey(pk) => EncryptionEngine::encrypt_with_key(&processed_data, pk)?,
                EncryptionMode::None => processed_data,
            };

            let size = processed_data.len() as u64;
            self.storage.write(&object_path, &mut std::io::Cursor::new(processed_data))?;
            Ok((hash, size, should_compress))
        } else {
            Ok((hash, 0, false)) // Already exists
        }
    }

    pub fn get_object(&self, hash: &str, mime_type: Option<&str>, is_compressed: bool) -> Result<Vec<u8>> {
        let is_encrypted = self.encryption.is_encrypted();
        let object_id = ObjectStoreKey::compute_object_id(hash, mime_type, is_encrypted);
        let object_path = ObjectStoreKey::compute_object_path(hash, &object_id);

        let mut reader = self.storage.read(&object_path)?;
        let mut data = Vec::new();
        use std::io::Read;
        reader.read_to_end(&mut data)?;

        if is_encrypted {
            data = match self.encryption {
                EncryptionMode::Password(pwd) => EncryptionEngine::decrypt(&data, pwd)?,
                EncryptionMode::PublicKey(sk) => EncryptionEngine::decrypt_with_key(&data, sk)?,
                EncryptionMode::None => anyhow::bail!("Data is encrypted but no key provided"),
            };
        }

        if is_compressed {
            data = CompressionEngine::decompress(&data)?;
        }

        Ok(data)
    }

    pub fn chunk_and_put(&self, data: &[u8]) -> Result<Vec<Chunk>> {
        let chunks = Chunker::chunk_data(data);
        let mut results = Vec::new();

        for (chunk_info, chunk_data) in chunks {
            // Chunks are never individually compressed for performance (too small), but always encrypted if requested.
            self.put_object(&chunk_data, None)?;
            results.push(chunk_info);
        }

        Ok(results)
    }
}
