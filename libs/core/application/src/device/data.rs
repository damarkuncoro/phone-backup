use crate::analysis::vcard::VCardEngine;
use crate::backup::BackupService;
use anyhow::Result;
use domain::{DeviceId, SnapshotId, StructuredDataType};
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};
use tracing::{info, instrument};

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
        info!("Fetching structured data '{}' for snapshot {}", data_type, snapshot_id.0);

        match data_type {
            StructuredDataType::Contacts => {
                let contacts = self.repository.get_snapshot_contacts(snapshot_id)?;
                Ok(serde_json::to_value(contacts)?)
            }
            StructuredDataType::Sms => {
                let sms = self.repository.get_snapshot_sms(snapshot_id)?;
                Ok(serde_json::to_value(sms)?)
            }
            StructuredDataType::CallLogs => {
                let logs = self.repository.get_snapshot_call_logs(snapshot_id)?;
                Ok(serde_json::to_value(logs)?)
            }
            _ => Ok(serde_json::Value::Null),
        }
    }

    #[instrument(skip(self))]
    pub fn export_contacts_vcard(&self, snapshot_id: &SnapshotId) -> Result<String> {
        let contacts = self.repository.get_snapshot_contacts(snapshot_id)?;
        Ok(VCardEngine::export_to_vcard(&contacts))
    }

    #[instrument(skip(self))]
    pub fn export_contacts_csv(&self, snapshot_id: &SnapshotId) -> Result<String> {
        let domain_contacts = self.repository.get_snapshot_contacts(snapshot_id)?;
        let mut book_builder = contacts::ContactBook::builder();
        for dc in domain_contacts {
            let mut cb = contacts::ContactBuilder::new(&dc.display_name);
            if let Some(sn) = dc.names.first() {
                cb = cb.with_structured_name(
                    contacts::StructuredName::new()
                        .with_given(sn.given_name.clone().unwrap_or_default())
                        .with_family(sn.family_name.clone().unwrap_or_default())
                        .with_prefix(sn.prefix.clone().unwrap_or_default())
                        .with_suffix(sn.suffix.clone().unwrap_or_default()),
                );
            }
            for p in dc.phones {
                cb = cb.add_phone(p.raw_value, contacts::PhoneType::Mobile);
            }
            for e in dc.emails {
                cb = cb.add_email(e.value, contacts::EmailType::Personal);
            }
            if let Some(org) = dc.organizations.first() {
                cb = cb.with_organization(org.company_name.clone().unwrap_or_default(), org.title.as_deref());
            }
            if let Some(note) = dc.notes {
                cb = cb.with_notes(note);
            }
            book_builder = book_builder.add_contact(cb.build());
        }
        let book = book_builder.build();
        book.export(contacts::ExportFormat::Csv)
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
