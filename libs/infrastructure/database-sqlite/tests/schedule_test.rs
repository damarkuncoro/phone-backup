mod common;

use chrono::Utc;
use common::setup_test_repo;
use domain::{BackupSchedule, ConnectionType, Device, DeviceId, ScheduleFrequency};
use ports::{DeviceRepositoryPort, ScheduleRepositoryPort};

#[test]
fn test_schedule_crud() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-sched");

    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "A".to_string(),
        model: "B".to_string(),
        serial: "C".to_string(),
        os_version: "D".to_string(),
        sdk_version: None,
        storage_total_bytes: 0,
        storage_used_bytes: 0,
        storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    })
    .unwrap();

    // CREATE
    let schedule = BackupSchedule {
        device_id: device_id.clone(),
        frequency: ScheduleFrequency::Daily,
        last_run_at: None,
        enabled: true,
    };
    repo.save_schedule(&schedule).unwrap();

    // READ
    let saved = repo
        .get_schedule(&device_id)
        .unwrap()
        .expect("Schedule should exist");
    assert_eq!(saved.frequency, ScheduleFrequency::Daily);
    assert!(saved.enabled);

    // UPDATE
    let mut updated = saved;
    updated.frequency = ScheduleFrequency::Weekly;
    updated.last_run_at = Some(Utc::now());
    repo.save_schedule(&updated).unwrap();

    let after_update = repo.get_schedule(&device_id).unwrap().unwrap();
    assert_eq!(after_update.frequency, ScheduleFrequency::Weekly);
    assert!(after_update.last_run_at.is_some());

    // LIST
    let enabled = repo.list_schedules().unwrap();
    assert_eq!(enabled.len(), 1);
}
