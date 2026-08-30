use chrono::Utc;
use phone_backup_domain::{
    BackupPolicy, BackupSchedule, Checksum, DeviceId, DevicePath, KeepCountStrategy,
    KeepDailyStrategy, RetentionStrategy, ScheduleFrequency, Snapshot, SnapshotStatus, StorageSize,
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

#[test]
fn test_checksum_value_object() {
    let valid = Checksum::new("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert!(valid.is_ok());
    assert_eq!(valid.unwrap().as_str(), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");

    let invalid_short = Checksum::new("invalid_hex");
    assert!(invalid_short.is_err());
}

#[test]
fn test_storage_size_formatting() {
    let bytes = StorageSize::from_bytes(1024);
    assert_eq!(bytes.format_human_readable(), "1.00 KB");

    let mb = StorageSize::from_bytes(1024 * 1024 * 5);
    assert_eq!(mb.format_human_readable(), "5.00 MB");

    let gb = StorageSize::from_bytes(1024 * 1024 * 1024 * 2);
    assert_eq!(gb.format_human_readable(), "2.00 GB");
}

#[test]
fn test_device_path_security() {
    let valid = DevicePath::new("sdcard/DCIM/Camera/photo.jpg");
    assert!(valid.is_ok());
    assert_eq!(valid.unwrap().as_str(), "/sdcard/DCIM/Camera/photo.jpg");

    let traversal = DevicePath::new("sdcard/DCIM/../../../etc/passwd");
    assert!(traversal.is_err());
}

struct TestEventHandler {
    received_events: std::sync::Arc<std::sync::Mutex<Vec<phone_backup_domain::DomainEvent>>>,
}

impl phone_backup_domain::DomainEventHandler for TestEventHandler {
    fn handle(&self, event: &phone_backup_domain::DomainEvent) {
        self.received_events.lock().unwrap().push(event.clone());
    }
}

#[test]
fn test_domain_event_bus_pub_sub() {
    use phone_backup_domain::{DomainEvent, DomainEventBus, EventHandlerRef};
    use std::sync::{Arc, Mutex};

    let bus = DomainEventBus::new();
    let events = Arc::new(Mutex::new(Vec::new()));

    let handler: EventHandlerRef = Arc::new(TestEventHandler {
        received_events: events.clone(),
    });

    bus.subscribe(handler);
    assert_eq!(bus.handler_count(), 1);

    let event = DomainEvent::DeviceConnected {
        device_id: DeviceId::new("DEV_PUB_SUB"),
        timestamp: Utc::now(),
    };

    bus.publish(&event);

    let received = events.lock().unwrap();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0], event);
}

#[test]
fn test_scan_result_warning_aggregation() {
    use phone_backup_domain::{ScanResult, ScanWarning, ScanSource};

    let warning = ScanWarning {
        source: ScanSource::FileSystem,
        path: "/sdcard/Android/data".to_string(),
        message: "Permission denied (Scoped Storage)".to_string(),
    };

    let result = ScanResult::new(vec![], vec![warning.clone()]);
    assert!(!result.is_successful());
    assert_eq!(result.warning_count(), 1);
    assert_eq!(result.file_count(), 0);
    assert_eq!(result.warnings[0], warning);
}

#[test]
fn test_structured_data_type_formatting() {
    use phone_backup_domain::StructuredDataType;

    assert_eq!(StructuredDataType::Contacts.as_str(), "contacts");
    assert_eq!(StructuredDataType::Sms.as_str(), "sms");
    assert_eq!(StructuredDataType::CallLogs.as_str(), "call_logs");
    assert_eq!(StructuredDataType::Applications.as_str(), "apps");
    assert_eq!(format!("{}", StructuredDataType::Contacts), "contacts");
}
