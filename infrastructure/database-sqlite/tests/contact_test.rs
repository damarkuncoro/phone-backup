mod common;

use common::setup_test_repo;
use domain::{Device, DeviceId, ConnectionType, Snapshot, SnapshotId, SnapshotStatus, Contact, ContactPhone, ContactEmail};
use ports::{DeviceRepositoryPort, SnapshotRepositoryPort, ContactRepositoryPort};
use chrono::Utc;

#[test]
fn test_contact_complex_persistence() {
    let repo = setup_test_repo();
    let device_id = DeviceId::new("dev-contact");
    let snap_id = SnapshotId("snap-contact".to_string());

    // Setup Device & Snapshot
    repo.save_device(&Device {
        id: device_id.clone(),
        manufacturer: "X".to_string(), model: "Y".to_string(), serial: "Z".to_string(),
        os_version: "13".to_string(), sdk_version: None,
        storage_total_bytes: 0, storage_used_bytes: 0, storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    }).unwrap();

    repo.create_snapshot(&Snapshot {
        id: snap_id.clone(), device_id: device_id.clone(), started_at: Utc::now(),
        finished_at: None, status: SnapshotStatus::Pending, total_files: 0,
        total_bytes: 0, deduped_bytes: 0,
    }).unwrap();

    // CREATE Contact with multiple sub-entities
    let contact = Contact {
        id: "source-123".to_string(),
        snapshot_id: None,
        source_id: Some("123".to_string()),
        display_name: "John Doe".to_string(),
        notes: Some("Friend".to_string()),
        source: "google".to_string(),
        source_account: Some("john@gmail.com".to_string()),
        content_hash: Some("abc".to_string()),
        metadata_json: None,
        names: vec![],
        phones: vec![
            ContactPhone { raw_value: "+12345".to_string(), normalized_value: Some("12345".to_string()), phone_type: Some("mobile".to_string()), label: None, is_primary: true },
            ContactPhone { raw_value: "+54321".to_string(), normalized_value: Some("54321".to_string()), phone_type: Some("home".to_string()), label: None, is_primary: false },
        ],
        emails: vec![
            ContactEmail { value: "john@example.com".to_string(), email_type: Some("work".to_string()), label: None, is_primary: true },
        ],
        addresses: vec![],
        organizations: vec![],
        urls: vec![],
        events: vec![],
        photos: vec![],
        labels: vec!["Friends".to_string(), "Work".to_string()],
    };

    repo.save_contact(&snap_id, &contact).unwrap();

    // READ & VERIFY
    let contacts = repo.get_snapshot_contacts(&snap_id).unwrap();
    assert_eq!(contacts.len(), 1);

    let saved = &contacts[0];
    assert_eq!(saved.display_name, "John Doe");
    assert_eq!(saved.phones.len(), 2);
    assert_eq!(saved.emails.len(), 1);
    assert_eq!(saved.labels.len(), 2);

    // Verify specific data
    let primary_phone = saved.phones.iter().find(|p| p.is_primary).unwrap();
    assert_eq!(primary_phone.raw_value, "+12345");
}

#[test]
fn test_contact_search() {
    let repo = setup_test_repo();
    let snap_id = SnapshotId("s1".to_string());

    // We need device/snapshot for FKs
    repo.save_device(&Device {
        id: DeviceId::new("d1"), manufacturer: "A".to_string(), model: "B".to_string(), serial: "C".to_string(),
        os_version: "D".to_string(), sdk_version: None,
        storage_total_bytes: 0, storage_used_bytes: 0, storage_free_bytes: 0,
        connection_type: ConnectionType::Usb,
    }).unwrap();

    repo.create_snapshot(&Snapshot {
        id: snap_id.clone(), device_id: DeviceId::new("d1"), started_at: Utc::now(),
        finished_at: None, status: SnapshotStatus::Pending, total_files: 0,
        total_bytes: 0, deduped_bytes: 0,
    }).unwrap();

    let c1 = Contact {
        id: "1".to_string(), snapshot_id: None, source_id: None, display_name: "Alice Smith".to_string(),
        notes: None, source: "s".to_string(), source_account: None, content_hash: None, metadata_json: None,
        names: vec![], phones: vec![], emails: vec![], addresses: vec![], organizations: vec![],
        urls: vec![], events: vec![], photos: vec![], labels: vec![],
    };
    let c2 = Contact {
        id: "2".to_string(), snapshot_id: None, source_id: None, display_name: "Bob Jones".to_string(),
        notes: None, source: "s".to_string(), source_account: None, content_hash: None, metadata_json: None,
        names: vec![], phones: vec![], emails: vec![], addresses: vec![], organizations: vec![],
        urls: vec![], events: vec![], photos: vec![], labels: vec![],
    };

    repo.save_contact(&snap_id, &c1).unwrap();
    repo.save_contact(&snap_id, &c2).unwrap();

    let results = repo.search_contacts("Smith").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1.display_name, "Alice Smith");
}
