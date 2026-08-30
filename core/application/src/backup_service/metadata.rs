use anyhow::Result;
use domain::{DeviceId, EncryptionMode, Snapshot};
use ports::{AppProviderPort, DataProviderPort, RepositoryPort, StoragePort, ProgressPort, DevicePort, ScannerPort};
use super::BackupService;

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
        tracing::info!("Starting app list backup...");
        if let Ok(apps) = self.app_provider.list_apps(id) {
            for app in &apps {
                let _ = self.repository.save_app(app);
                let _ = self.repository.link_app_to_snapshot(&snapshot.id, &app.id);
            }
            tracing::info!("Backed up {} apps", apps.len());
        }

        tracing::info!("Starting structured data backup (Contacts, SMS, Logs)...");
        if let Ok(contacts) = self.data_provider.list_contacts(id) {
            let _ = self.store_structured_data(&snapshot.id, "contacts", &contacts, encryption);
            for contact in contacts {
                let _ = self.repository.save_contact(&snapshot.id, &contact);
            }
        }

        if let Ok(sms) = self.data_provider.list_sms(id) {
            let _ = self.store_structured_data(&snapshot.id, "sms", &sms, encryption);
            let _ = self.repository.save_sms_batch(&snapshot.id, &sms);
        }

        if let Ok(logs) = self.data_provider.list_call_logs(id) {
            let _ = self.store_structured_data(&snapshot.id, "call_logs", &logs, encryption);
            let _ = self.repository.save_call_logs_batch(&snapshot.id, &logs);
        }
        Ok(())
    }
}
