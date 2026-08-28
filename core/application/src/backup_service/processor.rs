use anyhow::Result;
use domain::{EncryptionMode, FileEntry, DeviceId};
use ports::{DevicePort, StoragePort, RepositoryPort};
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::security::EncryptionEngine;
use crate::compression::CompressionEngine;
use crate::media_analysis::MediaAnalyzer;
use crate::hashing::calculate_hash;
use crate::object_store::ObjectStoreKey;
use crate::backup_service::BackupService;
use ports::{AppProviderPort, DataProviderPort, ScannerPort};

pub struct FileProcessor<'a, D, S, R, T, A, DP>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
{
    pub(crate) service: &'a BackupService<D, S, R, T, A, DP>,
    pub(crate) encryption: &'a EncryptionMode,
    pub(crate) total_bytes: &'a AtomicU64,
    pub(crate) total_files: &'a AtomicU64,
    pub(crate) deduped_bytes: &'a AtomicU64,
}

impl<'a, D, S, R, T, A, DP> FileProcessor<'a, D, S, R, T, A, DP>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
{
    pub fn process_file(&self, id: &DeviceId, mut file: FileEntry, skip_content: bool) -> Result<FileEntry> {
        if !skip_content {
            match self.service.device_adapter.read_file(id, &file.path) {
                Ok(mut content_reader) => {
                    let mut content_buf = Vec::with_capacity(file.size_bytes as usize);
                    content_reader.read_to_end(&mut content_buf)?;

                    let hash = calculate_hash(&content_buf);
                    file.hash_sha256 = Some(hash.clone());
                    file.media_info = MediaAnalyzer::extract_info(&content_buf, &file.mime_type);

                    let object_id = ObjectStoreKey::compute_object_id(&hash, Some(&file.mime_type), self.encryption.is_encrypted());
                    let object_path = ObjectStoreKey::compute_object_path(&hash, &object_id);

                    if !self.service.storage.exists(&object_path)? {
                        let mut data_to_write = content_buf;
                        if CompressionEngine::should_compress(&file.mime_type) {
                            data_to_write = CompressionEngine::compress(&data_to_write)?;
                        }

                        data_to_write = match &self.encryption {
                            EncryptionMode::Password(pwd) => EncryptionEngine::encrypt(&data_to_write, pwd)?,
                            EncryptionMode::PublicKey(pk) => EncryptionEngine::encrypt_with_key(&data_to_write, pk)?,
                            EncryptionMode::None => data_to_write,
                        };

                        self.service.storage.write(&object_path, &mut std::io::Cursor::new(data_to_write))?;
                    } else {
                        self.deduped_bytes.fetch_add(file.size_bytes, Ordering::Relaxed);
                    }
                }
                Err(e) => return Err(anyhow::anyhow!("Device read error: {}", e)),
            }
        }

        self.service.repository.save_file(&file)?;
        self.total_bytes.fetch_add(file.size_bytes, Ordering::Relaxed);
        self.total_files.fetch_add(1, Ordering::Relaxed);

        Ok(file)
    }
}
