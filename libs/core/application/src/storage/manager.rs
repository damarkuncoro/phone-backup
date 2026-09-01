use anyhow::Result;
use domain::EncryptionMode;
use ports::{StoragePort, RepositoryPort};
use super::{Chunker, Chunk, ChunkConfig, ChunkingMethod, CompressionAlgorithm, CompressionEngine, EncryptionEngine};
use super::store::ObjectStoreKey;
use super::hashing::calculate_hash;

pub struct ObjectManager<'a, T: StoragePort, R: RepositoryPort> {
    storage: &'a T,
    repository: &'a R,
    encryption: &'a EncryptionMode,
}

impl<'a, T: StoragePort, R: RepositoryPort> ObjectManager<'a, T, R> {
    pub fn new(storage: &'a T, repository: &'a R, encryption: &'a EncryptionMode) -> Self {
        Self { storage, repository, encryption }
    }

    /// V4.0 Object Processing: Hashing -> Dedup -> Compress -> Encrypt -> Store
    /// Returns (logical_chunk_id, was_new_physical_object)
    pub fn put_chunk(&self, data: &[u8]) -> Result<(String, bool)> {
        let content_hash = calculate_hash(data);
        let plaintext_size = data.len() as u64;

        // 1. Logical Dedup: Check if this content hash already exists
        if let Some(chunk_id) = self.repository.get_logical_chunk_by_hash(&content_hash)? {
            // Check if we have at least one physical object for it
            if let Some(_) = self.repository.get_storage_key_for_chunk(&chunk_id)? {
                return Ok((chunk_id, false));
            }
            // If no physical object exists (unlikely but possible), continue to create one
            return self.create_physical_object(&chunk_id, data);
        }

        // 2. New Content: Create Logical Chunk
        let chunk_id = self.repository.save_logical_chunk(&content_hash, plaintext_size)?;
        self.create_physical_object(&chunk_id, data)
    }

    fn create_physical_object(&self, chunk_id: &str, data: &[u8]) -> Result<(String, bool)> {
        let mut processed_data = data.to_vec();
        let mut comp_alg = "none".to_string();

        // 3. Compress (Optional)
        if data.len() > 1024 {
            processed_data = CompressionEngine::compress(&processed_data, CompressionAlgorithm::Zstd)?;
            comp_alg = "zstd".to_string();
        }

        // 4. Encrypt (Optional but recommended in V4.0)
        let _enc_version = if self.encryption.is_encrypted() { 1 } else { 0 };
        processed_data = match self.encryption {
            EncryptionMode::Password(pwd) => EncryptionEngine::encrypt(&processed_data, pwd)?,
            EncryptionMode::PublicKey(pk) => EncryptionEngine::encrypt_with_key(&processed_data, pk)?,
            EncryptionMode::None => processed_data,
        };

        // 5. Calculate Physical ID (Ciphertext Hash)
        let object_hash = calculate_hash(&processed_data);
        let stored_size = processed_data.len() as u64;

        // 6. Final Dedup Check (Physical level)
        if let Some(_) = self.repository.get_physical_object_by_hash(&object_hash)? {
            return Ok((chunk_id.to_string(), false));
        }

        // 7. Store Physically (UUIDv7)
        let storage_key = ObjectStoreKey::generate_storage_key();
        let storage_path = ObjectStoreKey::compute_object_path_v4(&storage_key);

        self.storage.write(&storage_path, &mut std::io::Cursor::new(processed_data))?;

        // 8. Register Physical Object
        self.repository.save_physical_object(
            chunk_id,
            &object_hash,
            &storage_key,
            stored_size,
            &comp_alg,
            if self.encryption.is_encrypted() { 1 } else { 0 }
        )?;

        Ok((chunk_id.to_string(), true))
    }

    pub fn get_chunk(&self, chunk_id: &str) -> Result<Vec<u8>> {
        // Find storage key
        let storage_key = self.repository.get_storage_key_for_chunk(chunk_id)?
            .ok_or_else(|| anyhow::anyhow!("Chunk object not found for ID: {}", chunk_id))?;

        let storage_path = ObjectStoreKey::compute_object_path_v4(&storage_key);
        let mut reader = self.storage.read(&storage_path)?;
        let mut data = Vec::new();
        use std::io::Read;
        reader.read_to_end(&mut data)?;

        if self.encryption.is_encrypted() {
            data = match self.encryption {
                EncryptionMode::Password(pwd) => EncryptionEngine::decrypt(&data, pwd)?,
                EncryptionMode::PublicKey(sk) => EncryptionEngine::decrypt_with_key(&data, sk)?,
                EncryptionMode::None => anyhow::bail!("Data is encrypted but no key provided"),
            };
        }

        // Decompress if it looks like zstd or based on metadata
        // In expert mode, we use the strategy. For now fallback to zstd if it fails none
        if let Ok(decompressed) = CompressionEngine::decompress(&data, CompressionAlgorithm::Zstd) {
            data = decompressed;
        }

        Ok(data)
    }

    /// Legacy support or simple object put (used for thumbnails, etc.)
    pub fn put_object(&self, data: &[u8], _mime_type: Option<&str>) -> Result<(String, u64, bool)> {
        let (chunk_id, is_new) = self.put_chunk(data)?;
        Ok((chunk_id, if is_new { data.len() as u64 } else { 0 }, false))
    }

    /// Returns (chunks, bytes_reused)
    pub fn chunk_and_put(&self, data: &[u8], method: ChunkingMethod, config: ChunkConfig) -> Result<(Vec<Chunk>, u64)> {
        let chunks = Chunker::chunk_data(data, method, config)?;
        let mut results = Vec::new();
        let mut reused_bytes = 0;

        for (mut chunk_info, chunk_data) in chunks {
            let (chunk_id, is_new) = self.put_chunk(&chunk_data)?;
            if !is_new {
                reused_bytes += chunk_info.length as u64;
            }
            chunk_info.hash = chunk_id;
            results.push(chunk_info);
        }

        Ok((results, reused_bytes))
    }

    /// Returns (chunks, bytes_reused)
    pub fn chunk_and_put_stream<R2: std::io::Read + 'static>(
        &self,
        reader: R2,
        method: ChunkingMethod,
        config: ChunkConfig
    ) -> Result<(Vec<Chunk>, u64)> {
        let mut results = Vec::new();
        let mut reused_bytes = 0;
        let mut stream = Chunker::create_stream(reader, method, config);

        while let Some((mut chunk_info, chunk_data)) = stream.next_chunk()? {
            let (chunk_id, is_new) = self.put_chunk(&chunk_data)?;
            if !is_new {
                reused_bytes += chunk_info.length as u64;
            }
            chunk_info.hash = chunk_id;
            results.push(chunk_info);
        }

        Ok((results, reused_bytes))
    }
}
