use anyhow::Result;
use domain::{DeviceId, FileEntry, FileId};
use mtp_rs::{ObjectHandle, Storage};

use super::paths::MtpPathResolver;
use super::session::NativeMtpOperations;

pub struct MtpDirectoryOps;

impl MtpDirectoryOps {
    pub fn list_directory(
        session: &NativeMtpOperations,
        device_id: &DeviceId,
        path: &str,
    ) -> Result<Vec<FileEntry>> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        rt.block_on(async {
            let device = session.get_device().await?;
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
            let (storage, handle, _) = MtpPathResolver::resolve_storage_and_handle(&device, path).await?;
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
                    mime_type: if is_dir { "inode/directory".into() } else { "application/octet-stream".into() },
                    permissions: if is_dir { "drwxr-xr-x".into() } else { "-rw-r--r--".into() },
                    hash_sha256: None,
                    thumbnail_hash: None,
                    media_info: None,
                });
            }
            Ok(entries)
        })
    }

    pub fn scan_recursive(
        session: &NativeMtpOperations,
        device_id: &DeviceId,
        target_paths: Vec<String>,
    ) -> Result<Vec<FileEntry>> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        rt.block_on(async {
            let device = session.get_device().await?;
            let storages = device.storages().await?;
            let mut all_files = Vec::new();
            let paths = if target_paths.is_empty() { vec!["/".to_string()] } else { target_paths };

            for start_path in paths {
                if start_path == "/" {
                    for storage in &storages {
                        let desc = storage.info().description.clone();
                        Self::scan_internal(device_id, storage, None, &format!("/{}", desc), &mut all_files).await?;
                    }
                } else if let Ok((storage, handle, _)) = MtpPathResolver::resolve_storage_and_handle(&device, &start_path).await {
                    Self::scan_internal(device_id, &storage, handle, &start_path, &mut all_files).await?;
                }
            }
            Ok(all_files)
        })
    }

    async fn scan_internal(
        device_id: &DeviceId,
        storage: &Storage,
        parent: Option<ObjectHandle>,
        current_path: &str,
        results: &mut Vec<FileEntry>,
    ) -> Result<()> {
        let mut stack = vec![(parent, current_path.to_string())];
        while let Some((p_handle, p_path)) = stack.pop() {
            if p_path.ends_with("/Android/data") || p_path.ends_with("/Android/obb") || p_path.ends_with("/.trashBin_File") {
                tracing::debug!("Skipping restricted Android folder: {}", p_path);
                continue;
            }
            let items = match storage.list_objects(p_handle).await {
                Ok(it) => it,
                Err(e) => {
                    tracing::warn!("Skipping folder '{}': {}", p_path, e);
                    continue;
                }
            };
            for item in items {
                let info = match storage.get_object_info(item.handle).await {
                    Ok(inf) => inf,
                    Err(e) => {
                        tracing::warn!("Skipping item '{}/{}': {}", p_path, item.filename, e);
                        continue;
                    }
                };
                let virtual_path = format!("{}/{}", p_path.trim_end_matches('/'), item.filename);
                if info.format.is_association() {
                    stack.push((Some(item.handle), virtual_path));
                } else {
                    let mime = mime_guess::from_path(&item.filename).first_or_octet_stream().to_string();
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
}
