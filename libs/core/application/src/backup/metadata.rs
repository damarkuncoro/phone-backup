use super::BackupService;
use anyhow::Result;
use domain::{DeviceId, EncryptionMode, Snapshot};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};

impl<
        D: DevicePort,
        S: ScannerPort,
        R: RepositoryPort,
        T: StoragePort,
        A: AppProviderPort,
        DP: DataProviderPort,
        P: ProgressPort,
    > BackupService<D, S, R, T, A, DP, P>
{
    pub(crate) fn backup_metadata_and_structured_data(
        &self,
        id: &DeviceId,
        snapshot: &mut Snapshot,
        encryption: &EncryptionMode,
    ) -> Result<()> {
        self.progress.log("Backing up applications list...");
        if let Ok(apps) = self.app_provider.list_apps(id) {
            for app in &apps {
                let _ = self.repository.save_app(app);
                let _ = self.repository.link_app_to_snapshot(&snapshot.id, &app.id);
            }
            self.progress.log(&format!("Backed up {} apps", apps.len()));
        }

        self.progress
            .log("Backing up structured data (Contacts, SMS)...");
        if let Ok(contacts) = self.data_provider.list_contacts(id) {
            self.progress
                .log(&format!("Saving {} contacts...", contacts.len()));
            let _ = self.store_structured_data(
                &snapshot.id,
                domain::StructuredDataType::Contacts,
                &contacts,
                encryption,
            );
            for contact in contacts {
                let _ = self.repository.save_contact(&snapshot.id, &contact);
            }
        }

        if let Ok(sms) = self.data_provider.list_sms(id) {
            self.progress
                .log(&format!("Saving {} messages...", sms.len()));
            let _ = self.store_structured_data(
                &snapshot.id,
                domain::StructuredDataType::Sms,
                &sms,
                encryption,
            );
            let _ = self.repository.save_sms_batch(&snapshot.id, &sms);
        }

        if let Ok(logs) = self.data_provider.list_call_logs(id) {
            self.progress
                .log(&format!("Saving {} call logs...", logs.len()));
            let _ = self.store_structured_data(
                &snapshot.id,
                domain::StructuredDataType::CallLogs,
                &logs,
                encryption,
            );
            let _ = self.repository.save_call_logs_batch(&snapshot.id, &logs);
        }
        Ok(())
    }

    pub(crate) fn store_structured_data<V: serde::Serialize>(
        &self,
        snapshot_id: &domain::SnapshotId,
        data_type: domain::StructuredDataType,
        data: &V,
        encryption: &EncryptionMode,
    ) -> Result<()> {
        let json = serde_json::to_vec(data)?;
        let object_manager = crate::storage::manager::ObjectManager::new(
            &self.storage,
            &self.repository,
            encryption,
        );

        let (chunk_id, _, _) = object_manager.put_object(&json, None)?;

        self.repository
            .save_structured_data_ref(snapshot_id, data_type, &chunk_id)?;
        Ok(())
    }

    pub(crate) fn check_battery_and_thermal(&self, id: &DeviceId) -> Result<()> {
        if let Ok((level, temp)) = self.device_adapter.battery_status(id) {
            if level < 2 {
                anyhow::bail!("Battery critically low ({}%). Please charge your device.", level);
            } else if level < 10 {
                tracing::warn!("Safety Notice: Battery is low ({}%), but device is connected via USB. Proceeding.", level);
            }
            if temp > 45.0 {
                anyhow::bail!(
                    "Device temperature too high ({:.1}°C). Let it cool down.",
                    temp
                );
            }
            tracing::info!("Safety Check: Battery {}%, Temp {}°C - OK", level, temp);
        }
        Ok(())
    }

    pub(crate) fn check_available_disk_space(&self, required_bytes: u64) -> Result<()> {
        let available = self.storage.available_space()?;
        if available < required_bytes {
            anyhow::bail!(
                "Insufficient disk space on target storage. Required: {:.2} MB, Available: {:.2} MB",
                required_bytes as f64 / 1024.0 / 1024.0,
                available as f64 / 1024.0 / 1024.0
            );
        }
        tracing::info!(
            "Target Storage Capacity Check: OK (Available: {:.2} MB, Required: {:.2} MB)",
            available as f64 / 1024.0 / 1024.0,
            required_bytes as f64 / 1024.0 / 1024.0
        );
        Ok(())
    }
}
