use crate::backup::BackupService;
use anyhow::Result;
use domain::{AppInfo, DeviceId, FileEntry, Snapshot, SnapshotId, SnapshotStatus};
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
    pub fn list_snapshots(&self, id: &DeviceId) -> Result<Vec<Snapshot>> {
        self.repository.list_snapshots(id)
    }

    #[instrument(skip(self))]
    pub fn get_latest_snapshot_any_device(&self) -> Result<Option<Snapshot>> {
        let devices = self.list_devices()?;
        let mut latest: Option<Snapshot> = None;
        for d in devices {
            if let Ok(snapshots) = self.list_snapshots(&d.id) {
                if let Some(s) = snapshots
                    .into_iter()
                    .find(|s| s.status == SnapshotStatus::Completed)
                {
                    if latest.is_none() || s.started_at > latest.as_ref().unwrap().started_at {
                        latest = Some(s);
                    }
                }
            }
        }
        Ok(latest)
    }

    #[instrument(skip(self))]
    pub fn get_snapshot(&self, id: &SnapshotId) -> Result<Option<Snapshot>> {
        self.repository.get_snapshot(id)
    }

    #[instrument(skip(self))]
    pub fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> Result<Vec<AppInfo>> {
        self.repository.get_snapshot_apps(snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> Result<Vec<FileEntry>> {
        self.repository.get_snapshot_files(snapshot_id)
    }

    #[instrument(skip(self))]
    pub fn delete_snapshot(&self, id: &SnapshotId) -> Result<()> {
        self.repository.delete_snapshot(id)
    }

    #[instrument(skip(self))]
    pub fn prune_failed_snapshots(&self) -> Result<usize> {
        let snapshots = self.repository.list_all_snapshots()?;
        let mut deleted_count = 0;

        for s in snapshots {
            if s.status != SnapshotStatus::Completed {
                info!(
                    "Pruning incomplete/failed snapshot: {} (status: {:?})",
                    s.id.0, s.status
                );
                self.delete_snapshot(&s.id)?;
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }
}
