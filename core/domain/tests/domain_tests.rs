use chrono::Utc;
use phone_backup_domain::{
    BackupPolicy, BackupSchedule, DeviceId, KeepCountStrategy, KeepDailyStrategy,
    RetentionStrategy, ScheduleFrequency, Snapshot, SnapshotStatus,
};

#[test]
fn test_builder_pattern() {
    let policy = BackupPolicy::builder()
        .include("/sdcard/DCIM")
        .exclude("*.bak")
        .build();

    assert!(policy.should_include("/sdcard/DCIM/photo.jpg"));
    assert!(!policy.should_include("/sdcard/Downloads/file.pdf"));
    assert!(!policy.should_include("/sdcard/DCIM/photo.bak"));
}

#[test]
fn test_include_policy() {
    let policy = BackupPolicy::builder().include("/sdcard/DCIM").build();

    assert!(policy.should_include("/sdcard/DCIM/photo.jpg"));
    assert!(!policy.should_include("/sdcard/Downloads/file.pdf"));
}

#[test]
fn test_exclude_policy() {
    let policy = BackupPolicy::builder()
        .exclude("*.tmp")
        .exclude("cache/")
        .build();

    assert!(!policy.should_include("/sdcard/data.tmp"));
    assert!(!policy.should_include("/sdcard/Android/cache/info.log"));
    assert!(policy.should_include("/sdcard/Documents/notes.txt"));
}

#[test]
fn test_keep_count_strategy() {
    let dev_id = DeviceId::new("DEV1");
    let mut snapshots = Vec::new();

    for i in 0..5 {
        let mut s = Snapshot::new(dev_id.clone());
        s.status = SnapshotStatus::Completed;
        s.started_at = Utc::now() - chrono::Duration::hours(i);
        snapshots.push(s);
    }

    let strategy = KeepCountStrategy { keep_limit: 2 };
    let to_delete = strategy.select_snapshots_to_delete(&snapshots);

    assert_eq!(to_delete.len(), 3);
}

#[test]
fn test_keep_daily_strategy() {
    let dev_id = DeviceId::new("DEV1");
    let mut snapshots = Vec::new();

    for i in 0..5 {
        let mut s = Snapshot::new(dev_id.clone());
        s.status = SnapshotStatus::Completed;
        s.started_at = Utc::now() - chrono::Duration::days(i);
        snapshots.push(s);
    }

    let strategy = KeepDailyStrategy { keep_days: 3 };
    let to_delete = strategy.select_snapshots_to_delete(&snapshots);

    assert_eq!(to_delete.len(), 2);
}

#[test]
fn test_schedule_on_connect() {
    let schedule = BackupSchedule {
        device_id: DeviceId("dev-1".to_string()),
        frequency: ScheduleFrequency::OnConnect,
        last_run_at: None,
        enabled: true,
    };
    assert!(schedule.is_due());

    let schedule_ran = BackupSchedule {
        device_id: DeviceId("dev-1".to_string()),
        frequency: ScheduleFrequency::OnConnect,
        last_run_at: Some(Utc::now()),
        enabled: true,
    };
    assert!(!schedule_ran.is_due());
}
