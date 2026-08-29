use anyhow::Result;
use chrono::Utc;
use domain::{
    BackupSchedule, DeviceId, KeepDailyStrategy, RetentionPolicy, RetentionStrategy, ScheduleFrequency, EncryptionMode,
};
use ports::{AppProviderPort, DataProviderPort, DevicePort, RepositoryPort, ScannerPort, StoragePort};

use tracing::{error, info, instrument};

use super::BackupService;

impl<
        D: DevicePort,
        S: ScannerPort,
        R: RepositoryPort,
        T: StoragePort,
        A: AppProviderPort,
        DP: DataProviderPort,
        P: ports::ProgressPort,
    > BackupService<D, S, R, T, A, DP, P>
{
    #[instrument(skip(self))]
    pub fn apply_retention_policy(&self, device_id: &DeviceId, policy: RetentionPolicy) -> Result<u32> {
        let strategy = KeepDailyStrategy {
            keep_days: policy.keep_daily,
        };
        self.apply_retention_strategy(device_id, &strategy)
    }

    #[instrument(skip(self, strategy))]
    pub fn apply_retention_strategy(
        &self,
        device_id: &DeviceId,
        strategy: &dyn RetentionStrategy,
    ) -> Result<u32> {
        let snapshots = self.repository.list_snapshots(device_id)?;
        let to_delete = strategy.select_snapshots_to_delete(&snapshots);

        let mut deleted_count = 0;
        for s_id in &to_delete {
            info!("Auto-cleanup: Deleting old snapshot {} (Retention Strategy)", s_id.0);
            self.repository.delete_snapshot(s_id)?;
            deleted_count += 1;
        }

        Ok(deleted_count)
    }

    #[instrument(skip(self))]
    pub fn add_schedule(&self, device_id: DeviceId, frequency: ScheduleFrequency) -> Result<()> {
        let schedule = BackupSchedule {
            device_id,
            frequency,
            last_run_at: None,
            enabled: true,
        };
        self.repository.save_schedule(&schedule)
    }

    #[instrument(skip(self))]
    pub fn list_schedules(&self) -> Result<Vec<BackupSchedule>> {
        self.repository.list_schedules()
    }

    #[instrument(skip(self, encryption))]
    pub fn run_pending_backups(&self, encryption: EncryptionMode) -> Result<()> {
        let schedules = self.repository.list_schedules()?;
        let connected_devices = self.device_adapter.discover()?;

        for mut schedule in schedules {
            if schedule.is_due() {
                if let Some(device) = connected_devices.iter().find(|d| d.id == schedule.device_id) {
                    info!("Running scheduled backup for device {} ({})", device.model, schedule.device_id);

                    // Mark as attempted to avoid immediate retry loop on failure
                    schedule.last_run_at = Some(Utc::now());
                    let _ = self.repository.save_schedule(&schedule);

                    self.progress.start(1, &format!("Auto-backup: {}", device.model));

                    match self.perform_backup(&schedule.device_id, encryption.clone(), None) {
                        Ok(_) => {
                            info!("Auto-backup for {} completed successfully", device.model);
                            self.progress.finish(&format!("Auto-backup for {} completed", device.model));
                        }
                        Err(e) => {
                            error!("Scheduled backup failed for {}: {}", schedule.device_id, e);
                            // We already updated last_run_at, so it won't retry until next interval (Hourly/Daily)
                            self.progress.error(&format!("Auto-backup for {} failed: {}", device.model, e));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
