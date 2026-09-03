use domain::{Capability, CapabilityStatus, ConnectionType, DeviceId};
use phone_backup_adapter_ios::{IosDeviceAdapter, IosDeviceProperties};
use ports::DevicePort;

#[test]
fn test_ios_properties_conversion() {
    let props = IosDeviceProperties {
        unique_device_id: "00008110-001234567890ABCD".to_string(),
        device_name: "Damar's iPhone".to_string(),
        product_type: "iPhone15,3".to_string(),
        product_version: "18.1".to_string(),
        serial_number: Some("F2L1234567".to_string()),
        total_disk_capacity: Some(256 * 1024 * 1024 * 1024),
        total_data_available: Some(180 * 1024 * 1024 * 1024),
    };

    assert_eq!(props.get_marketing_name(), "iPhone 14 Pro Max");

    let device = props.to_device();
    assert_eq!(device.manufacturer, "Apple");
    assert_eq!(device.model, "iPhone 14 Pro Max");
    assert_eq!(device.os_version, "iOS 18.1");
    assert_eq!(device.connection_type, ConnectionType::Usb);

    let caps = props.to_capability_matrix();
    assert_eq!(caps.status(Capability::ReadFiles), CapabilityStatus::Available);
    assert_eq!(caps.status(Capability::ReadMedia), CapabilityStatus::Available);
    assert_eq!(caps.status(Capability::ReadSms), CapabilityStatus::Unsupported);
}

#[test]
fn test_ios_device_adapter_mock() {
    let adapter = IosDeviceAdapter::new();
    let dev_id = DeviceId::new("00008110-001234567890ABCD");
    let dev = adapter.info(&dev_id).expect("info failed");
    assert_eq!(dev.manufacturer, "Apple");
}
