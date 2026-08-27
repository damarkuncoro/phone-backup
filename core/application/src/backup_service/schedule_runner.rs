use anyhow::Result;
use chrono::Utc;
use domain::{BackupSchedule, DeviceId, RetentionPolicy, ScheduleFrequency, SnapshotStatus};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};

use super::BackupService;

impl<
        D: DevicePort,
        S: ScannerPort,
        R: RepositoryPort,
        T: StoragePort,
        A: AppProviderPort,
        DP: DataProviderPort,
    > BackupService<D, S, R, T, A, DP>
{
    pub fn apply_retention_policy(&self, device_id: &DeviceId, policy: RetentionPolicy) -> Result<u32> {
        let snapshots = self.repository.list_snapshots(device_id)?;
        let mut completed_snapshots: Vec<_> = snapshots
            .into_iter()
            .filter(|s| s.status == SnapshotStatus::Completed)
            .collect();

        completed_snapshots.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        let mut deleted_count = 0;
        let limit = policy.keep_daily as usize;

        if completed_snapshots.len() > limit {
            for s in completed_snapshots.iter().skip(limit) {
                println!("Auto-cleanup: Deleting old snapshot {} (Retention)", s.id.0);
                self.repository.delete_snapshot(&s.id)?;
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }

    pub fn add_schedule(&self, device_id: DeviceId, frequency: ScheduleFrequency) -> Result<()> {
        let schedule = BackupSchedule {
            device_id,
            frequency,
            last_run_at: None,
            enabled: true,
        };
        self.repository.save_schedule(&schedule)
    }

    pub fn list_schedules(&self) -> Result<Vec<BackupSchedule>> {
        self.repository.list_schedules()
    }

    pub fn run_pending_backups(&self, password: Option<&str>) -> Result<()> {
        let schedules = self.repository.list_schedules()?;
        let connected_devices = self.device_adapter.discover()?;

        for schedule in schedules {
            if schedule.is_due() {
                if connected_devices.iter().any(|d| d.id == schedule.device_id) {
                    println!("Running scheduled backup for device {}...", schedule.device_id);
                    match self.perform_backup(&schedule.device_id, password, None) {
                        Ok(_) => {
                            let mut updated_schedule = schedule;
                            updated_schedule.last_run_at = Some(Utc::now());
                            self.repository.save_schedule(&updated_schedule)?;
                        }
                        Err(e) => {
                            eprintln!("Scheduled backup failed for {}: {}", schedule.device_id, e);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
