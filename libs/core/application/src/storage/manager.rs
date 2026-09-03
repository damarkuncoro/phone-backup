use super::hashing::calculate_hash;
use super::store::ObjectStoreKey;
use super::{
    Chunk, ChunkConfig, Chunker, ChunkingMethod, CompressionAlgorithm, EncryptionEngine,
    FileMetadataContext, SmartCompressionEngine,
};
use anyhow::Result;
use domain::EncryptionMode;
use ports::{RepositoryPort, StoragePort};

pub struct ObjectManager<'a, T: StoragePort, R: RepositoryPort> {
    storage: &'a T,
    repository: &'a R,
    encryption: &'a EncryptionMode,
}

impl<'a, T: StoragePort, R: RepositoryPort> ObjectManager<'a, T, R> {
    pub fn new(storage: &'a T, repository: &'a R, encryption: &'a EncryptionMode) -> Self {
        Self { storage, repository, encryption }
    }

    pub fn put_chunk(&self, data: &[u8]) -> Result<(String, bool)> {
        self.put_chunk_with_context(data, &FileMetadataContext::default())
    }

    pub fn put_chunk_with_context(&self, data: &[u8], ctx: &FileMetadataContext) -> Result<(String, bool)> {
        let content_hash = calculate_hash(data);
        let plaintext_size = data.len() as u64;

        if let Some(chunk_id) = self.repository.get_logical_chunk_by_hash(&content_hash)? {
            if self.repository.get_storage_key_for_chunk(&chunk_id)?.is_some() {
                return Ok((chunk_id, false));
            }
            return self.create_physical_object_with_context(&chunk_id, data, ctx);
        }

        let chunk_id = self.repository.save_logical_chunk(&content_hash, plaintext_size)?;
        self.create_physical_object_with_context(&chunk_id, data, ctx)
    }

    fn create_physical_object_with_context(
        &self,
        chunk_id: &str,
        data: &[u8],
        context: &FileMetadataContext,
    ) -> Result<(String, bool)> {
        let comp_engine = SmartCompressionEngine::builder().with_android_dictionaries().build();
        let (mut processed, stats) = comp_engine.compress(data, context)?;
        let comp_alg = match stats.algorithm {
            CompressionAlgorithm::Zstd => "zstd".to_string(),
            CompressionAlgorithm::None => "none".to_string(),
        };

        processed = match self.encryption {
            EncryptionMode::Password(pwd) => EncryptionEngine::encrypt(&processed, pwd)?,
            EncryptionMode::PublicKey(pk) => EncryptionEngine::encrypt_with_key(&processed, pk)?,
            EncryptionMode::None => processed,
        };

        let object_hash = calculate_hash(&processed);
        let stored_size = processed.len() as u64;

        if self.repository.get_physical_object_by_hash(&object_hash)?.is_some() {
            return Ok((chunk_id.to_string(), false));
        }

        let storage_key = ObjectStoreKey::generate_storage_key();
        let storage_path = ObjectStoreKey::compute_object_path_v4(&storage_key);
        self.storage.write(&storage_path, &mut std::io::Cursor::new(processed))?;

        self.repository.save_physical_object(
            chunk_id,
            &object_hash,
            &storage_key,
            stored_size,
            &comp_alg,
            if self.encryption.is_encrypted() { 1 } else { 0 },
        )?;

        Ok((chunk_id.to_string(), true))
    }

    pub fn get_chunk(&self, chunk_id: &str) -> Result<Vec<u8>> {
        let storage_key = self
            .repository
            .get_storage_key_for_chunk(chunk_id)?
            .ok_or_else(|| anyhow::anyhow!("Chunk object not found: {}", chunk_id))?;

        let storage_path = ObjectStoreKey::compute_object_path_v4(&storage_key);
        let mut reader = self.storage.read(&storage_path)?;
        let mut data = Vec::new();
        use std::io::Read;
        reader.read_to_end(&mut data)?;

        if self.encryption.is_encrypted() {
            data = match self.encryption {
                EncryptionMode::Password(pwd) => EncryptionEngine::decrypt(&data, pwd)?,
                EncryptionMode::PublicKey(sk) => EncryptionEngine::decrypt_with_key(&data, sk)?,
                EncryptionMode::None => anyhow::bail!("Encrypted data without key"),
            };
        }

        let comp_engine = SmartCompressionEngine::builder().with_android_dictionaries().build();
        if let Ok(decompressed) = comp_engine.decompress(&data, CompressionAlgorithm::Zstd) {
            data = decompressed;
        }

        Ok(data)
    }

    pub fn put_object(&self, data: &[u8], mime: Option<&str>) -> Result<(String, u64, bool)> {
        let ctx = match mime {
            Some(m) => FileMetadataContext::new().with_mime(m),
            None => FileMetadataContext::default(),
        };
        let (chunk_id, is_new) = self.put_chunk_with_context(data, &ctx)?;
        Ok((chunk_id, if is_new { data.len() as u64 } else { 0 }, false))
    }

    pub fn chunk_and_put(&self, data: &[u8], m: ChunkingMethod, c: ChunkConfig) -> Result<(Vec<Chunk>, u64)> {
        self.chunk_and_put_with_context(data, m, c, &FileMetadataContext::default())
    }

    pub fn chunk_and_put_with_context(
        &self,
        data: &[u8],
        method: ChunkingMethod,
        config: ChunkConfig,
        ctx: &FileMetadataContext,
    ) -> Result<(Vec<Chunk>, u64)> {
        let chunks = Chunker::chunk_data(data, method, config)?;
        let mut results = Vec::new();
        let mut reused_bytes = 0;

        for (mut chunk_info, chunk_data) in chunks {
            let (chunk_id, is_new) = self.put_chunk_with_context(&chunk_data, ctx)?;
            if !is_new {
                reused_bytes += chunk_info.length as u64;
            }
            chunk_info.hash = chunk_id;
            results.push(chunk_info);
        }

        Ok((results, reused_bytes))
    }

    pub fn chunk_and_put_stream<R2: std::io::Read + 'static>(
        &self,
        reader: R2,
        method: ChunkingMethod,
        config: ChunkConfig,
    ) -> Result<(Vec<Chunk>, u64)> {
        self.chunk_and_put_stream_with_context(reader, method, config, &FileMetadataContext::default())
    }

    pub fn chunk_and_put_stream_with_context<R2: std::io::Read + 'static>(
        &self,
        reader: R2,
        method: ChunkingMethod,
        config: ChunkConfig,
        ctx: &FileMetadataContext,
    ) -> Result<(Vec<Chunk>, u64)> {
        let mut results = Vec::new();
        let mut reused_bytes = 0;
        let mut stream = Chunker::create_stream(reader, method, config);

        while let Some((mut chunk_info, chunk_data)) = stream.next_chunk()? {
            let (chunk_id, is_new) = self.put_chunk_with_context(&chunk_data, ctx)?;
            if !is_new {
                reused_bytes += chunk_info.length as u64;
            }
            chunk_info.hash = chunk_id;
            results.push(chunk_info);
        }

        Ok((results, reused_bytes))
    }
}
