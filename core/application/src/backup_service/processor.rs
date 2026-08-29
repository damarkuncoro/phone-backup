use anyhow::Result;
use domain::{DeviceId, FileEntry};
use ports::{DevicePort, RepositoryPort, StoragePort, ScannerPort, AppProviderPort, DataProviderPort, ProgressPort};
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::media_analysis::MediaAnalyzer;
use crate::object_manager::ObjectManager;
use tracing::instrument;

pub struct FileProcessor<'a, D, S, R, T, A, DP, P>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
{
    pub(crate) service: &'a crate::backup_service::BackupService<D, S, R, T, A, DP, P>,
    pub(crate) object_manager: ObjectManager<'a, T>,
    pub(crate) total_bytes: &'a AtomicU64,
    pub(crate) total_files: &'a AtomicU64,
    pub(crate) deduped_bytes: &'a AtomicU64,
}

impl<'a, D, S, R, T, A, DP, P> FileProcessor<'a, D, S, R, T, A, DP, P>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
{
    #[instrument(skip(self, id, skip_content), fields(file = %file.path))]
    pub fn process_file(&self, id: &DeviceId, mut file: FileEntry, skip_content: bool) -> Result<FileEntry> {
        if !skip_content {
            let mut content_reader = self.service.device_adapter.read_file(id, &file.path)?;
            let mut content_buf = Vec::with_capacity(file.size_bytes as usize);
            content_reader.read_to_end(&mut content_buf)?;

            file.media_info = MediaAnalyzer::extract_info(&content_buf, &file.mime_type);

            if file.size_bytes > 4 * 1024 * 1024 {
                let chunks = self.object_manager.chunk_and_put(&content_buf)?;
                // For chunked files, we store the file hash as the aggregate hash if needed,
                // but usually CAS uses the chunk hashes. Let's still set the file hash.
                file.hash_sha256 = Some(crate::hashing::calculate_hash(&content_buf));

                // Save file record before chunks due to FK constraints
                self.service.repository.save_file(&file)?;

                for (i, chunk) in chunks.into_iter().enumerate() {
                    self.service.repository.save_file_chunk(&file.id, &chunk.hash, chunk.offset, chunk.length, i as u32)?;
                }
            } else {
                let (hash, stored_size, _) = self.object_manager.put_object(&content_buf, Some(&file.mime_type))?;
                file.hash_sha256 = Some(hash);
                if stored_size == 0 {
                    self.deduped_bytes.fetch_add(file.size_bytes, Ordering::Relaxed);
                }
                // Save file record
                self.service.repository.save_file(&file)?;
            }
        } else {
             self.service.repository.save_file(&file)?;
        }

        self.total_bytes.fetch_add(file.size_bytes, Ordering::Relaxed);
        self.total_files.fetch_add(1, Ordering::Relaxed);

        Ok(file)
    }
}
