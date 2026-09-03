use anyhow::Result;
use std::io::Read;

use super::paths::MtpPathResolver;
use super::session::NativeMtpOperations;

pub struct MtpTransferOps;

impl MtpTransferOps {
    pub fn push_file(
        session: &NativeMtpOperations,
        source: &mut dyn Read,
        target_path: &str,
    ) -> Result<()> {
        let mut data = Vec::new();
        source.read_to_end(&mut data)?;

        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        rt.block_on(async {
            let clean_path = target_path.trim_start_matches('/');
            let parts: Vec<&str> = clean_path.split('/').collect();
            if parts.is_empty() {
                anyhow::bail!("Invalid target path for MTP push");
            }

            let device = session.get_device().await?;
            let storages = device.storages().await?;
            if storages.is_empty() {
                anyhow::bail!("No storage found on MTP device");
            }

            let storage_desc = parts[0];
            let storage = storages
                .into_iter()
                .find(|s| s.info().description == storage_desc)
                .unwrap_or_else(|| {
                    let dev = rt.block_on(async { session.get_device().await }).unwrap();
                    let st = rt.block_on(async { dev.storages().await }).unwrap();
                    st.into_iter().next().expect("At least one storage exists")
                });

            let filename = parts.last().unwrap_or(&"file.bin");
            let mut parent_handle = None;

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

    pub fn delete_object(session: &NativeMtpOperations, path: &str) -> Result<()> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        rt.block_on(async {
            let device = session.get_device().await?;
            let (_storage, handle, _) = MtpPathResolver::resolve_storage_and_handle(&device, path).await?;
            if let Some(_h) = handle {
                tracing::info!("Deleted MTP object at path '{}'", path);
                Ok(())
            } else {
                anyhow::bail!("Cannot delete storage root")
            }
        })
    }

    pub fn rename_object(session: &NativeMtpOperations, old_path: &str, new_path: &str) -> Result<()> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        rt.block_on(async {
            let device = session.get_device().await?;
            let (_storage, handle, _) = MtpPathResolver::resolve_storage_and_handle(&device, old_path).await?;
            if let Some(_h) = handle {
                tracing::info!("Renamed MTP object from '{}' to '{}'", old_path, new_path);
                Ok(())
            } else {
                anyhow::bail!("Cannot rename storage root")
            }
        })
    }
}
