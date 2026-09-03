use crate::analysis::media::MediaAnalyzer;
use crate::storage::manager::ObjectManager;
use crate::storage::{Chunk, FileMetadataContext};
use anyhow::Result;
use domain::{DeviceId, FileEntry};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::instrument;

use crate::storage::policy::ChunkingPolicy;

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
    pub(crate) service: &'a super::BackupService<D, S, R, T, A, DP, P>,
    pub(crate) object_manager: ObjectManager<'a, T, R>,
    pub(crate) chunking_policy: Box<dyn ChunkingPolicy>,
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
    pub fn process_file(
        &self,
        id: &DeviceId,
        mut file: FileEntry,
        skip_content: bool,
    ) -> Result<(FileEntry, Vec<Chunk>)> {
        let mut chunks = Vec::new();

        if !skip_content {
            let (method, config) = self.chunking_policy.determine_strategy(&file);
            let mut content_reader = self.service.device_adapter.read_file(id, &file.path)?;

            let ext = file.name.rsplit('.').next().unwrap_or("");
            let context = FileMetadataContext::new()
                .with_mime(&file.mime_type)
                .with_extension(ext)
                .with_size(file.size_bytes as usize);

            // 1. If it's an image, load it for thumbnail extraction
            if file.mime_type.starts_with("image/") && file.size_bytes > 0 {
                let mut content_buf = Vec::with_capacity(file.size_bytes as usize);
                content_reader.read_to_end(&mut content_buf)?;

                file.media_info = MediaAnalyzer::extract_info(&content_buf, &file.mime_type);

                // Generate Thumbnail
                if let Ok(img) = image::load_from_memory(&content_buf) {
                    let thumbnail = img.thumbnail(256, 256);
                    let mut thumb_buf = std::io::Cursor::new(Vec::new());
                    if thumbnail
                        .write_to(&mut thumb_buf, image::ImageFormat::Jpeg)
                        .is_ok()
                    {
                        let data = thumb_buf.into_inner();
                        if let Ok((hash, _, _)) =
                            self.object_manager.put_object(&data, Some("image/jpeg"))
                        {
                            file.thumbnail_hash = Some(hash);
                        }
                    }
                }

                // Process image chunks with metadata context
                let (c, reused) = self.object_manager.chunk_and_put_with_context(
                    &content_buf,
                    method,
                    config,
                    &context,
                )?;
                chunks = c;
                file.hash_sha256 = Some(crate::storage::hashing::calculate_hash(&content_buf));
                self.deduped_bytes.fetch_add(reused, Ordering::Relaxed);
            } else {
                // 2. For non-images (Video, DB, Docs), stream with metadata context
                let (c, reused) = self.object_manager.chunk_and_put_stream_with_context(
                    content_reader,
                    method,
                    config,
                    &context,
                )?;
                chunks = c;
                self.deduped_bytes.fetch_add(reused, Ordering::Relaxed);

                if chunks.len() == 1 && method == crate::storage::ChunkingMethod::FullFile {
                    file.hash_sha256 = Some(chunks[0].hash.clone());
                }
            }
        }

        self.total_bytes
            .fetch_add(file.size_bytes, Ordering::Relaxed);
        self.total_files.fetch_add(1, Ordering::Relaxed);

        Ok((file, chunks))
    }
}
