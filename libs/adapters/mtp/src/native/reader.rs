use anyhow::{anyhow, Result};
use mtp_rs::{ObjectHandle, Storage};
use std::io::{Cursor, Read};

use super::paths::MtpPathResolver;
use super::session::NativeMtpOperations;

pub struct MtpReaderOps;

impl MtpReaderOps {
    pub fn read_file(session: &NativeMtpOperations, path: &str) -> Result<Box<dyn Read>> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        let (storage, handle) = rt.block_on(async {
            let device = session.get_device().await?;
            let (storage, handle, _) = MtpPathResolver::resolve_storage_and_handle(&device, path).await?;
            let h = handle.ok_or_else(|| anyhow!("Cannot read a storage root as a file"))?;
            Ok::<_, anyhow::Error>((storage, h))
        })?;

        Ok(Box::new(MtpStreamingReader::new(storage, handle, rt)))
    }

    pub fn calculate_quick_hash(session: &NativeMtpOperations, path: &str) -> Result<String> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        rt.block_on(async {
            let device = session.get_device().await?;
            let (storage, handle, _) = MtpPathResolver::resolve_storage_and_handle(&device, path).await?;
            let h = handle.ok_or_else(|| anyhow!("File not found or is a directory"))?;

            let info = storage.get_object_info(h).await?;
            let size = info.size;

            if size < 2 * 1024 * 1024 {
                let data = storage.download_to_vec(h).await?;
                return Ok(blake3::hash(&data).to_string());
            }

            let head = storage.read_range(h, 0, 1024 * 1024).await?;
            let tail = storage.read_range(h, size - (1024 * 1024), 1024 * 1024).await?;

            let mut hasher = blake3::Hasher::new();
            hasher.update(&head);
            hasher.update(&tail);
            hasher.update(&size.to_le_bytes());

            Ok(hasher.finalize().to_string())
        })
    }
}

pub struct MtpStreamingReader {
    storage: Storage,
    handle: ObjectHandle,
    rt: tokio::runtime::Runtime,
    buffer: Cursor<Vec<u8>>,
    offset: u64,
    total_size: u64,
    eof: bool,
}

impl MtpStreamingReader {
    pub fn new(storage: Storage, handle: ObjectHandle, rt: tokio::runtime::Runtime) -> Self {
        let info = rt.block_on(async { storage.get_object_info(handle).await }).unwrap();
        Self {
            storage,
            handle,
            rt,
            buffer: Cursor::new(Vec::new()),
            offset: 0,
            total_size: info.size,
            eof: false,
        }
    }

    fn fetch_next_chunk(&mut self) -> Result<bool> {
        if self.offset >= self.total_size {
            return Ok(false);
        }
        let chunk_size = 1024 * 1024;
        let remaining = self.total_size - self.offset;
        let to_read = std::cmp::min(chunk_size, remaining);

        let data = self.rt.block_on(async {
            self.storage
                .read_range(self.handle, self.offset, to_read as u32)
                .await
                .map_err(|e| anyhow!("MTP Read Error at {}: {:?}", self.offset, e))
        })?;

        self.offset += data.len() as u64;
        self.buffer = Cursor::new(data);
        Ok(true)
    }
}

impl Read for MtpStreamingReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.eof {
            return Ok(0);
        }
        let mut n = self.buffer.read(buf)?;
        if n == 0 {
            match self.fetch_next_chunk() {
                Ok(true) => {
                    n = self.buffer.read(buf)?;
                }
                Ok(false) => {
                    self.eof = true;
                    return Ok(0);
                }
                Err(e) => return Err(std::io::Error::other(e)),
            }
        }
        Ok(n)
    }
}
