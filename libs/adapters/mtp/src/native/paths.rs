use anyhow::{anyhow, Result};
use mtp_rs::{MtpDevice, ObjectHandle, Storage};

pub struct MtpPathResolver;

impl MtpPathResolver {
    /// Helper to resolve which storage and what handle a path refers to.
    /// Supports virtual root for multi-storage devices.
    pub async fn resolve_storage_and_handle(
        device: &MtpDevice,
        path: &str,
    ) -> Result<(Storage, Option<ObjectHandle>, String)> {
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

        let storage = storages
            .into_iter()
            .find(|s| s.info().description == storage_name)
            .ok_or_else(|| anyhow!("Storage '{}' not found.", storage_name))?;

        let mut current_handle = None;
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
}
