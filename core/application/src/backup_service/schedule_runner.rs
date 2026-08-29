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
    > BackupService<D, S, R, T, A, DP>
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

        for schedule in schedules {
            if schedule.is_due() {
                if connected_devices.iter().any(|d| d.id == schedule.device_id) {
                    info!("Running scheduled backup for device {}...", schedule.device_id);
                    match self.perform_backup(&schedule.device_id, encryption.clone(), None) {
                        Ok(_) => {
                            let mut updated_schedule = schedule;
                            updated_schedule.last_run_at = Some(Utc::now());
                            self.repository.save_schedule(&updated_schedule)?;
                        }
                        Err(e) => {
                            error!("Scheduled backup failed for {}: {}", schedule.device_id, e);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
