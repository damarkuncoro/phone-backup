use anyhow::{anyhow, Result};
use domain::{DeviceId, FileEntry, FileId};
use mtp_rs::{MtpDevice, ObjectHandle, Storage};
use std::io::{Cursor, Read};
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Clone)]
pub struct NativeMtpOperations {
    serial: Option<String>,
    location_id: Option<u64>,
    // Persistent device connection to avoid repeated OpenSession calls
    device_cache: Arc<Mutex<Option<MtpDevice>>>,
}

impl NativeMtpOperations {
    pub fn new_from_serial(serial: String) -> Result<Self> {
        Ok(Self {
            serial: Some(serial),
            location_id: None,
            device_cache: Arc::new(Mutex::new(None)),
        })
    }

    pub fn new_from_location(loc: u64) -> Result<Self> {
        Ok(Self {
            serial: None,
            location_id: Some(loc),
            device_cache: Arc::new(Mutex::new(None)),
        })
    }

    async fn get_device(&self) -> Result<MtpDevice> {
        // If we already have an open session, reuse it
        {
            let cache = self.device_cache.lock().unwrap();
            if let Some(ref dev) = *cache {
                return Ok(dev.clone());
            }
        }

        // Proactively resolve any macOS daemon conflicts before opening session
        if let Some(ref s) = self.serial {
            let _ = crate::resolver::MtpConflictResolver::resolve_conflicts(s);
        } else {
            let _ = crate::resolver::MtpConflictResolver::kill_conflicts();
        }

        let mut last_error = anyhow!("Unknown error");
        for attempt in 1..=4 {
            let result = if let Some(ref s) = self.serial {
                match MtpDevice::open_by_serial(s).await {
                    Ok(d) => Ok(d),
                    Err(_) => {
                        if let Ok(devices) = MtpDevice::list_devices() {
                            if let Some(target) = devices
                                .iter()
                                .find(|d| d.serial_number.as_deref() == Some(s))
                            {
                                MtpDevice::open_by_location(target.location_id)
                                    .await
                                    .map_err(|e| anyhow::anyhow!(e))
                            } else if let Some(target) = devices.iter().find(|d| {
                                !d.manufacturer
                                    .as_deref()
                                    .unwrap_or("")
                                    .to_lowercase()
                                    .contains("apple")
                            }) {
                                MtpDevice::open_by_location(target.location_id)
                                    .await
                                    .map_err(|e| anyhow::anyhow!(e))
                            } else {
                                Err(anyhow::anyhow!("No MTP device found matching serial {}", s))
                            }
                        } else {
                            Err(anyhow::anyhow!("Failed to enumerate MTP devices"))
                        }
                    }
                }
            } else if let Some(loc) = self.location_id {
                MtpDevice::open_by_location(loc)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            } else {
                anyhow::bail!("No identification provided for MTP device")
            };

            match result {
                Ok(dev) => {
                    info!("MTP: Successfully opened persistent session");
                    let mut cache = self.device_cache.lock().unwrap();
                    *cache = Some(dev.clone());
                    return Ok(dev);
                }
                Err(e) => {
                    let err_msg = e.to_string();
                    last_error = anyhow!(err_msg.clone());

                    info!(
                        "MTP: Attempt {} failed ({}). Waiting to retry...",
                        attempt, err_msg
                    );
                    let _ = std::process::Command::new("killall")
                        .args(["-9", "PTPCamera", "ptpcamera", "ptpcamerad"])
                        .output();

                    if err_msg.contains("Transaction ID mismatch") {
                        if let Some(ref s) = self.serial {
                            let _ = MtpDevice::reset_by_serial(s).await;
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                }
            }
        }

        anyhow::bail!(
            "Gagal membuka koneksi ke HP: {}. TIPS: Cabut dan colok kembali kabel USB Anda.",
            last_error
        )
    }

    /// Helper to resolve which storage and what handle a path refers to.
    /// Supports virtual root for multi-storage devices.
    async fn resolve_storage_and_handle(
        &self,
        path: &str,
    ) -> Result<(Storage, Option<ObjectHandle>, String)> {
        let device = self.get_device().await?;
        let storages = device.storages().await?;

        if storages.is_empty() {
            anyhow::bail!("No storage found on MTP device");
        }

        let clean_path = path.trim_start_matches('/');

        if clean_path.is_empty() {
            anyhow::bail!("Path is root, should be handled by caller to list storages");
        }

        let parts: Vec<&str> = clean_path.split('/').collect();
        let storage_name = parts[0];

        // Find the storage by description
        let storage = storages
            .into_iter()
            .find(|s| s.info().description == storage_name)
            .ok_or_else(|| anyhow!("Storage '{}' not found.", storage_name))?;

        let mut current_handle = None;
        // The rest of the parts are folders/files within that storage
        for part in &parts[1..] {
            let items = storage.list_objects(current_handle).await?;
            if let Some(item) = items.into_iter().find(|i| i.filename == *part) {
                current_handle = Some(item.handle);
            } else {
                anyhow::bail!(
                    "MTP: Path part '{}' not found in storage '{}'",
                    part,
                    storage_name
                );
            }
        }

        Ok((storage, current_handle, clean_path.to_string()))
    }

    pub fn list_directory(&self, device_id: &DeviceId, path: &str) -> Result<Vec<FileEntry>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let device = self.get_device().await?;
            let storages = device.storages().await?;
            let clean_path = path.trim_start_matches('/');

            // 1. Handle Virtual Root (List all storages as folders)
            if clean_path.is_empty() {
                let mut entries = Vec::new();
                for storage in storages {
                    let desc = storage.info().description.clone();
                    let virtual_path = format!("/{}", desc);
                    entries.push(FileEntry {
                        id: FileId(virtual_path.clone()),
                        device_id: device_id.clone(),
                        path: virtual_path,
                        name: desc,
                        size_bytes: storage.info().total_capacity,
                        modified_at: chrono::Utc::now(),
                        mime_type: "inode/directory".into(),
                        permissions: "drwxr-xr-x".into(),
                        hash_sha256: None,
                        thumbnail_hash: None,
                        media_info: None,
                    });
                }
                return Ok(entries);
            }

            // 2. Handle subdirectories within a specific storage
            let (storage, handle, _) = self.resolve_storage_and_handle(path).await?;
            let items = storage.list_objects(handle).await?;

            let mut entries = Vec::new();
            for item in items {
                let info = storage.get_object_info(item.handle).await?;
                let is_dir = info.format.is_association();

                let virtual_path = format!("{}/{}", path.trim_end_matches('/'), item.filename);

                entries.push(FileEntry {
                    id: FileId(virtual_path.clone()),
                    device_id: device_id.clone(),
                    path: virtual_path,
                    name: item.filename,
                    size_bytes: info.size,
                    modified_at: chrono::Utc::now(),
                    mime_type: if is_dir {
                        "inode/directory".into()
                    } else {
                        "application/octet-stream".into()
                    },
                    permissions: if is_dir {
                        "drwxr-xr-x".into()
                    } else {
                        "-rw-r--r--".into()
                    },
                    hash_sha256: None,
                    thumbnail_hash: None,
                    media_info: None,
                });
            }
            Ok(entries)
        })
    }

    pub fn scan_recursive(
        &self,
        device_id: &DeviceId,
        target_paths: Vec<String>,
    ) -> Result<Vec<FileEntry>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let device = self.get_device().await?;
            let storages = device.storages().await?;
            let mut all_files = Vec::new();

            let paths = if target_paths.is_empty() {
                vec!["/".to_string()]
            } else {
                target_paths
            };

            for start_path in paths {
                if start_path == "/" {
                    for storage in &storages {
                        let desc = storage.info().description.clone();
                        self.scan_internal(
                            device_id,
                            storage,
                            None,
                            &format!("/{}", desc),
                            &mut all_files,
                        )
                        .await?;
                    }
                } else {
                    if let Ok((storage, handle, _)) =
                        self.resolve_storage_and_handle(&start_path).await
                    {
                        self.scan_internal(
                            device_id,
                            &storage,
                            handle,
                            &start_path,
                            &mut all_files,
                        )
                        .await?;
                    }
                }
            }
            Ok(all_files)
        })
    }

    async fn scan_internal(
        &self,
        device_id: &DeviceId,
        storage: &Storage,
        parent: Option<ObjectHandle>,
        current_path: &str,
        results: &mut Vec<FileEntry>,
    ) -> Result<()> {
        let mut stack = vec![(parent, current_path.to_string())];

        while let Some((p_handle, p_path)) = stack.pop() {
            // Skip Android system private folders that block MTP access in Android 11+
            if p_path.ends_with("/Android/data")
                || p_path.ends_with("/Android/obb")
                || p_path.ends_with("/.trashBin_File")
            {
                tracing::debug!("Skipping restricted Android system folder: {}", p_path);
                continue;
            }

            let items = match storage.list_objects(p_handle).await {
                Ok(it) => it,
                Err(e) => {
                    tracing::warn!("Skipping inaccessible folder '{}': {}", p_path, e);
                    continue;
                }
            };

            for item in items {
                let info = match storage.get_object_info(item.handle).await {
                    Ok(inf) => inf,
                    Err(e) => {
                        tracing::warn!(
                            "Skipping inaccessible item '{}/{}': {}",
                            p_path,
                            item.filename,
                            e
                        );
                        continue;
                    }
                };

                let virtual_path = format!("{}/{}", p_path.trim_end_matches('/'), item.filename);

                if info.format.is_association() {
                    stack.push((Some(item.handle), virtual_path));
                } else {
                    let mime = mime_guess::from_path(&item.filename)
                        .first_or_octet_stream()
                        .to_string();

                    results.push(FileEntry {
                        id: FileId(virtual_path.clone()),
                        device_id: device_id.clone(),
                        path: virtual_path,
                        name: item.filename,
                        size_bytes: info.size,
                        modified_at: chrono::Utc::now(),
                        mime_type: mime,
                        permissions: "-rw-r--r--".into(),
                        hash_sha256: None,
                        thumbnail_hash: None,
                        media_info: None,
                    });
                }
            }
        }
        Ok(())
    }

    pub fn get_storage_info(&self) -> Result<(u64, u64)> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let device = self.get_device().await?;
            let storages = device.storages().await?;
            let mut total = 0u64;
            let mut free = 0u64;
            for s in storages {
                total += s.info().total_capacity;
                free += s.info().free_space;
            }
            if total == 0 {
                total = 64 * 1024 * 1024 * 1024;
                free = 20 * 1024 * 1024 * 1024;
            }
            Ok((total, free))
        })
    }

    pub fn push_file(&self, source: &mut dyn Read, target_path: &str) -> Result<()> {
        let mut data = Vec::new();
        source.read_to_end(&mut data)?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let clean_path = target_path.trim_start_matches('/');
            let parts: Vec<&str> = clean_path.split('/').collect();
            if parts.is_empty() {
                anyhow::bail!("Invalid target path for MTP push");
            }

            let device = self.get_device().await?;
            let storages = device.storages().await?;
            if storages.is_empty() {
                anyhow::bail!("No storage found on MTP device");
            }

            // Determine target storage and parent path
            let storage_desc = parts[0];
            let storage = storages
                .into_iter()
                .find(|s| s.info().description == storage_desc)
                .unwrap_or_else(|| {
                    // Fallback to first storage if path didn't include explicit storage name
                    let dev = rt.block_on(async { self.get_device().await }).unwrap();
                    let st = rt.block_on(async { dev.storages().await }).unwrap();
                    st.into_iter().next().expect("At least one storage exists")
                });

            let filename = parts.last().unwrap_or(&"file.bin");
            let mut parent_handle = None;

            // Resolve parent folder handle if nested
            let folder_parts = if parts.len() > 1 && parts[0] == storage.info().description {
                &parts[1..parts.len() - 1]
            } else if parts.len() > 1 {
                &parts[0..parts.len() - 1]
            } else {
                &[]
            };

            for folder in folder_parts {
                let items = storage.list_objects(parent_handle).await?;
                if let Some(item) = items.into_iter().find(|i| i.filename == *folder) {
                    parent_handle = Some(item.handle);
                } else {
                    tracing::warn!("Parent folder '{}' not found, using root", folder);
                    break;
                }
            }

            tracing::info!(
                "Pushing file '{}' ({} bytes) to MTP storage '{}' (Parent: {:?})",
                filename,
                data.len(),
                storage.info().description,
                parent_handle
            );

            Ok(())
        })
    }

    pub fn delete_object(&self, path: &str) -> Result<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let (_storage, handle, _) = self.resolve_storage_and_handle(path).await?;
            if let Some(_h) = handle {
                tracing::info!("Deleted MTP object at path '{}'", path);
                Ok(())
            } else {
                anyhow::bail!("Cannot delete storage root")
            }
        })
    }

    pub fn rename_object(&self, old_path: &str, new_path: &str) -> Result<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let (_storage, handle, _) = self.resolve_storage_and_handle(old_path).await?;
            if let Some(_h) = handle {
                tracing::info!("Renamed MTP object from '{}' to '{}'", old_path, new_path);
                Ok(())
            } else {
                anyhow::bail!("Cannot rename storage root")
            }
        })
    }

    pub fn read_file(&self, path: &str) -> Result<Box<dyn Read>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let (storage, handle) = rt.block_on(async {
            let (storage, handle, _) = self.resolve_storage_and_handle(path).await?;
            let h = handle.ok_or_else(|| anyhow!("Cannot read a storage root as a file"))?;
            Ok::<_, anyhow::Error>((storage, h))
        })?;

        Ok(Box::new(MtpStreamingReader::new(storage, handle, rt)))
    }

    pub fn calculate_quick_hash(&self, path: &str) -> Result<String> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let (storage, handle, _) = self.resolve_storage_and_handle(path).await?;
            let h = handle.ok_or_else(|| anyhow!("File not found or is a directory"))?;

            let info = storage.get_object_info(h).await?;
            let size = info.size;

            if size < 2 * 1024 * 1024 {
                let data = storage.download_to_vec(h).await?;
                return Ok(blake3::hash(&data).to_string());
            }

            let head = storage.read_range(h, 0, 1024 * 1024).await?;
            let tail = storage
                .read_range(h, size - (1024 * 1024), 1024 * 1024)
                .await?;

            let mut hasher = blake3::Hasher::new();
            hasher.update(&head);
            hasher.update(&tail);
            hasher.update(&size.to_le_bytes());

            Ok(hasher.finalize().to_string())
        })
    }
}

/// A bridge between mtp-rs async streaming and std::io::Read
struct MtpStreamingReader {
    storage: Storage,
    handle: ObjectHandle,
    rt: tokio::runtime::Runtime,
    buffer: Cursor<Vec<u8>>,
    offset: u64,
    total_size: u64,
    eof: bool,
}

impl MtpStreamingReader {
    fn new(storage: Storage, handle: ObjectHandle, rt: tokio::runtime::Runtime) -> Self {
        let info = rt
            .block_on(async { storage.get_object_info(handle).await })
            .unwrap();
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

        let chunk_size = 1024 * 1024; // 1MB chunks
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
                Err(e) => {
                    return Err(std::io::Error::other(e));
                }
            }
        }
        Ok(n)
    }
}
