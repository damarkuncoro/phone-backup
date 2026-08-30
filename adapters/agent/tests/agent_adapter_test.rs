use domain::{DeviceId, StructuredDataType};
use phone_backup_adapter_agent::{
    AgentAdapter, AgentHandshake, AgentSessionManager, AgentStructuredDataResponse,
};
use ports::{AppProviderPort, DataProviderPort, DevicePort, ScannerPort};

#[test]
fn test_agent_adapter_device_lifecycle() {
    let session = AgentSessionManager::new();
    let adapter = AgentAdapter::new(session.clone());

    // Initially empty
    let devices = adapter.discover().expect("Failed to discover");
    assert!(devices.is_empty());

    // Register an active companion agent
    session.register_device(AgentHandshake {
        device_id: "WIFI_DEV_999".to_string(),
        manufacturer: "Samsung".to_string(),
        model: "Galaxy S24 Ultra".to_string(),
        android_version: "Android 15".to_string(),
        storage_used_bytes: 50_000_000_000,
        storage_total_bytes: 512_000_000_000,
        capabilities: vec!["ReadFiles".into(), "ReadContacts".into()],
        battery_percent: Some(88),
        temperature_c: Some(31.2),
    });

    let devices = adapter.discover().expect("Failed to discover after register");
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].id.0, "WIFI_DEV_999");
    assert_eq!(devices[0].model, "Galaxy S24 Ultra");

    let info = adapter
        .info(&DeviceId("WIFI_DEV_999".into()))
        .expect("Device info failed");
    assert_eq!(info.model, "Galaxy S24 Ultra");

    let (battery, temp) = adapter
        .battery_status(&DeviceId("WIFI_DEV_999".into()))
        .expect("Battery status failed");
    assert_eq!(battery, 88);
    assert_eq!(temp, 31.2);
}

#[test]
fn test_agent_adapter_scanner_and_structured_data() {
    let adapter = AgentAdapter::with_default_session();
    let dev_id = DeviceId("AGENT_WIRELESS_01".into());

    // Test remote scan
    let files = adapter.scan(&dev_id, vec![]).expect("Scan failed");
    assert!(!files.is_empty());
    assert_eq!(files[0].path, "Pictures/agent_photo.jpg");

    // Test contacts extraction
    let contacts = adapter.list_contacts(&dev_id).expect("Contacts failed");
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0].display_name, "Damar Kuncoro (Wireless)");
    assert_eq!(contacts[0].phones[0].raw_value, "+6285921495599");

    // Test apps extraction
    let apps = adapter.list_apps(&dev_id).expect("Apps failed");
    assert_eq!(apps.len(), 1);
    assert_eq!(apps[0].package_name, "com.phonebackup.agent");
}
