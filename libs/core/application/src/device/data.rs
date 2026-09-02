use crate::analysis::vcard::VCardEngine;
use crate::backup::BackupService;
use anyhow::Result;
use domain::{DeviceId, SnapshotId, StructuredDataType};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};
use tracing::{error, info, instrument, warn};

impl<D, S, R, T, A, DP, P> BackupService<D, S, R, T, A, DP, P>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
{
    #[instrument(skip(self))]
    pub fn list_contacts(&self, id: &DeviceId) -> Result<Vec<domain::Contact>> {
        self.data_provider.list_contacts(id)
    }

    #[instrument(skip(self))]
    pub fn list_sms(&self, id: &DeviceId) -> Result<Vec<domain::Sms>> {
        self.data_provider.list_sms(id)
    }

    #[instrument(skip(self))]
    pub fn list_call_logs(&self, id: &DeviceId) -> Result<Vec<domain::CallLog>> {
        self.data_provider.list_call_logs(id)
    }

    #[instrument(skip(self))]
    pub fn get_structured_data(
        &self,
        snapshot_id: &SnapshotId,
        data_type: StructuredDataType,
    ) -> Result<serde_json::Value> {
        info!(
            "Fetching structured data '{}' for snapshot {}",
            data_type, snapshot_id.0
        );

        if data_type == StructuredDataType::Contacts {
            let contacts = self.repository.get_snapshot_contacts(snapshot_id)?;
            return Ok(serde_json::to_value(contacts)?);
        }

        let chunk_id = self
            .repository
            .get_structured_data_ref(snapshot_id, data_type)?
            .ok_or_else(|| {
                warn!(
                    "Structured data '{}' reference not found in database",
                    data_type
                );
                anyhow::anyhow!("Data type {} not found for this snapshot", data_type)
            })?;

        info!("Reading data from storage for chunk: {}", chunk_id);
        // We need an EncryptionMode here... let's assume None for now if not available or pass it from caller
        // Actually, BackupService should probably store the EncryptionMode used for the snapshot
        let object_manager = crate::storage::manager::ObjectManager::new(
            &self.storage,
            &self.repository,
            &domain::EncryptionMode::None,
        );
        let data = object_manager.get_chunk(&chunk_id)?;

        info!("Parsing JSON data ({} bytes)", data.len());
        let json: serde_json::Value = serde_json::from_slice(&data).map_err(|e| {
            error!("JSON parse error: {}. Data might be encrypted.", e);
            e
        })?;
        Ok(json)
    }

    #[instrument(skip(self))]
    pub fn export_contacts_vcard(&self, snapshot_id: &SnapshotId) -> Result<String> {
        let contacts = self.repository.get_snapshot_contacts(snapshot_id)?;
        Ok(VCardEngine::export_to_vcard(&contacts))
    }

    #[instrument(skip(self))]
    pub fn get_snapshot_sms(&self, snapshot_id: &SnapshotId) -> Result<Vec<domain::Sms>> {
        self.repository.get_snapshot_sms(snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn export_sms_json(&self, snapshot_id: &SnapshotId) -> Result<String> {
        let sms_list = self.repository.get_snapshot_sms(snapshot_id)?;
        Ok(serde_json::to_string_pretty(&sms_list)?)
    }

    #[instrument(skip(self))]
    pub fn get_snapshot_call_logs(&self, snapshot_id: &SnapshotId) -> Result<Vec<domain::CallLog>> {
        self.repository.get_snapshot_call_logs(snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn export_call_logs_json(&self, snapshot_id: &SnapshotId) -> Result<String> {
        let logs = self.repository.get_snapshot_call_logs(snapshot_id)?;
        Ok(serde_json::to_string_pretty(&logs)?)
    }
}
