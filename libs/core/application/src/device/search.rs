use anyhow::Result;
use domain::{DeviceId, FileEntry, SnapshotId, FileDiff, ContactDiff};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort, ProgressPort};
use tracing::instrument;
use crate::backup::BackupService;

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
    pub fn search_files(&self, query: &str) -> Result<Vec<FileEntry>> {
        self.repository.search_files(query)
    }

    #[instrument(skip(self))]
    pub fn search_contacts(&self, query: &str) -> Result<Vec<(SnapshotId, domain::Contact)>> {
        self.repository.search_contacts(query)
    }

    #[instrument(skip(self))]
    pub fn search_sms(&self, query: &str) -> Result<Vec<(SnapshotId, domain::Sms)>> {
        self.repository.search_sms(query)
    }

    #[instrument(skip(self))]
    pub fn search_call_logs(&self, query: &str) -> Result<Vec<(SnapshotId, domain::CallLog)>> {
        self.repository.search_call_logs(query)
    }

    #[instrument(skip(self))]
    pub fn list_media_files(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
        self.repository.list_media_files(device_id)
    }

    #[instrument(skip(self))]
    pub fn get_file_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> Result<FileDiff> {
        self.repository.get_file_diff(old_snapshot_id, new_snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn get_contact_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> Result<ContactDiff> {
        self.repository.get_contact_diff(old_snapshot_id, new_snapshot_id)
    }
}
