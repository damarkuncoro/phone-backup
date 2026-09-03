use domain::{AppId, AppInfo, AppSettings, BackupSchedule, Contact, DeviceId, SnapshotId};
use ports::{
    AppRepositoryPort, CallLogRepositoryPort, ContactRepositoryPort, MaintenanceRepositoryPort,
    RepositoryPort, ScheduleRepositoryPort, SettingsRepositoryPort, SmsRepositoryPort,
};

use crate::facade::SqliteRepository;

impl AppRepositoryPort for SqliteRepository {
    fn save_app(&self, app: &AppInfo) -> anyhow::Result<()> {
        self.apps().save_app(app)
    }
    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &AppId) -> anyhow::Result<()> {
        self.apps().link_app_to_snapshot(snapshot_id, app_id)
    }
    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<AppInfo>> {
        self.apps().get_snapshot_apps(snapshot_id)
    }
}

impl ContactRepositoryPort for SqliteRepository {
    fn save_contact(&self, snapshot_id: &SnapshotId, contact: &Contact) -> anyhow::Result<()> {
        self.contacts().save_contact(snapshot_id, contact)
    }
    fn get_snapshot_contacts(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<Contact>> {
        self.contacts().get_snapshot_contacts(snapshot_id)
    }
    fn search_contacts(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, Contact)>> {
        self.contacts().search_contacts(query)
    }
    fn get_contact_diff(&self, old_snapshot_id: &SnapshotId, new_snapshot_id: &SnapshotId) -> anyhow::Result<domain::ContactDiff> {
        self.contacts().get_contact_diff(old_snapshot_id, new_snapshot_id)
    }
}

impl ScheduleRepositoryPort for SqliteRepository {
    fn save_schedule(&self, schedule: &BackupSchedule) -> anyhow::Result<()> {
        self.schedules().save_schedule(schedule)
    }
    fn get_schedule(&self, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> {
        self.schedules().get_schedule(device_id)
    }
    fn list_schedules(&self) -> anyhow::Result<Vec<BackupSchedule>> {
        self.schedules().list_schedules()
    }
}

impl SettingsRepositoryPort for SqliteRepository {
    fn save_settings(&self, settings: &AppSettings) -> anyhow::Result<()> {
        self.settings().save_settings(settings)
    }
    fn get_settings(&self) -> anyhow::Result<Option<AppSettings>> {
        self.settings().get_settings()
    }
}

impl MaintenanceRepositoryPort for SqliteRepository {
    fn get_all_referenced_hashes(&self) -> anyhow::Result<std::collections::HashSet<String>> {
        self.maintenance().get_all_referenced_hashes()
    }
    fn optimize(&self) -> anyhow::Result<()> {
        self.maintenance().optimize()
    }
    fn prune_orphans(&self) -> anyhow::Result<u64> {
        self.maintenance().prune_orphans()
    }
    fn create_database_backup(&self, destination_path: &str) -> anyhow::Result<()> {
        self.maintenance().create_database_backup(destination_path)
    }
}

impl SmsRepositoryPort for SqliteRepository {
    fn save_sms(&self, snapshot_id: &SnapshotId, sms: &domain::Sms) -> anyhow::Result<()> {
        self.communication().save_sms(snapshot_id, sms)
    }
    fn save_sms_batch(&self, snapshot_id: &SnapshotId, sms_list: &[domain::Sms]) -> anyhow::Result<()> {
        self.communication().save_sms_batch(snapshot_id, sms_list)
    }
    fn get_snapshot_sms(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<domain::Sms>> {
        self.communication().get_snapshot_sms(snapshot_id)
    }
    fn search_sms(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, domain::Sms)>> {
        self.communication().search_sms(query)
    }
}

impl CallLogRepositoryPort for SqliteRepository {
    fn save_call_log(&self, snapshot_id: &SnapshotId, log: &domain::CallLog) -> anyhow::Result<()> {
        self.communication().save_call_log(snapshot_id, log)
    }
    fn save_call_logs_batch(&self, snapshot_id: &SnapshotId, logs: &[domain::CallLog]) -> anyhow::Result<()> {
        self.communication().save_call_logs_batch(snapshot_id, logs)
    }
    fn get_snapshot_call_logs(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<domain::CallLog>> {
        self.communication().get_snapshot_call_logs(snapshot_id)
    }
    fn search_call_logs(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, domain::CallLog)>> {
        self.communication().search_call_logs(query)
    }
}

impl RepositoryPort for SqliteRepository {}
