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
        self.progress.log("Backing up applications list...");
        if let Ok(apps) = self.app_provider.list_apps(id) {
            for app in &apps {
                let _ = self.repository.save_app(app);
                let _ = self.repository.link_app_to_snapshot(&snapshot.id, &app.id);
            }
            self.progress.log(&format!("Backed up {} apps", apps.len()));
        }

        self.progress.log("Backing up structured data (Contacts, SMS)...");
        if let Ok(contacts) = self.data_provider.list_contacts(id) {
            self.progress.log(&format!("Saving {} contacts...", contacts.len()));
            let _ = self.store_structured_data(&snapshot.id, domain::StructuredDataType::Contacts, &contacts, encryption);
            for contact in contacts {
                let _ = self.repository.save_contact(&snapshot.id, &contact);
            }
        }

        if let Ok(sms) = self.data_provider.list_sms(id) {
            self.progress.log(&format!("Saving {} messages...", sms.len()));
            let _ = self.store_structured_data(&snapshot.id, domain::StructuredDataType::Sms, &sms, encryption);
            let _ = self.repository.save_sms_batch(&snapshot.id, &sms);
        }

        if let Ok(logs) = self.data_provider.list_call_logs(id) {
            self.progress.log(&format!("Saving {} call logs...", logs.len()));
            let _ = self.store_structured_data(&snapshot.id, domain::StructuredDataType::CallLogs, &logs, encryption);
            let _ = self.repository.save_call_logs_batch(&snapshot.id, &logs);
        }
        Ok(())
    }
}
