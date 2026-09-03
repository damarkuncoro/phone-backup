use domain::DeviceId;
use phone_backup_adapter_adb::parsers::app_parser::AppParser;
use phone_backup_adapter_adb::parsers::communication_parser::CommunicationParser;
use phone_backup_adapter_adb::parsers::contact_parser::ContactParser;
use phone_backup_adapter_adb::parsers::device_parser::DeviceParser;
use phone_backup_adapter_adb::parsers::media_parser::MediaParser;

#[test]
fn test_parse_devices_l() {
    let output = "List of devices attached\n\
                  fynrorjncy6x4xib       device usb:337641472X product:redmi_note_11_pro model:22101316G device:rock transport_id:1\n\
                  emulator-5554          device product:sdk_gphone64_arm64 model:sdk_gphone64_arm64 device:emu64a transport_id:2";

    let devices = DeviceParser::parse_devices_l(output);
    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].id.0, "fynrorjncy6x4xib");
    assert_eq!(devices[0].model, "22101316G");
    assert_eq!(devices[1].id.0, "emulator-5554");
}

#[test]
fn test_parse_df_output() {
    let output = "Filesystem           1K-blocks      Used Available Use% Mounted on\n\
                  /dev/block/dm-46     230232532  41282476 188950056  18% /data";

    let (total, used, free) = DeviceParser::parse_df_output(output);
    assert_eq!(total, 230232532 * 1024);
    assert_eq!(used, 41282476 * 1024);
    assert_eq!(free, 188950056 * 1024);
}

#[test]
fn test_parse_pm_list_apps() {
    let device_id = DeviceId::new("test");
    let output = "package:com.whatsapp versionCode:12345\n\
                  package:com.android.settings versionCode:34";
    let mut versions = std::collections::HashMap::new();
    versions.insert("com.whatsapp".to_string(), "2.24.18.77".to_string());

    let apps = AppParser::parse_pm_list_detailed(&device_id, output, &versions);
    assert_eq!(apps.len(), 2);
    assert_eq!(apps[0].package_name, "com.whatsapp");
    assert_eq!(apps[0].app_name, "WhatsApp");
    assert_eq!(apps[0].version_name, "2.24.18.77");
    assert_eq!(apps[0].version_code, 12345);
    assert_eq!(apps[1].package_name, "com.android.settings");
    assert_eq!(apps[1].version_code, 34);
}

#[test]
fn test_parse_sms() {
    let output = "Row: 0 address=+628123456, body=Hello Test, date=1725000000000, type=1\n\
                  Row: 1 address=MyBank, body=Your OTP is 1234, date=1725000060000, type=1";

    let sms = CommunicationParser::parse_sms(output);
    assert_eq!(sms.len(), 2);
    assert_eq!(sms[0].address, "MyBank"); // Sorted by date desc
    assert_eq!(sms[0].body, "Your OTP is 1234");
}

#[test]
fn test_parse_contacts() {
    let device_id = DeviceId::new("test");
    let output = "Row: 0 contact_id=1, display_name=John Doe, mimetype=vnd.android.cursor.item/name, data1=John Doe, data2=John, data3=Doe\n\
                  Row: 1 contact_id=1, display_name=John Doe, mimetype=vnd.android.cursor.item/phone_v2, data1=0812345, data2=2";

    let contacts = ContactParser::parse(&device_id, output);
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0].display_name, "John Doe");
    assert_eq!(contacts[0].phones.len(), 1);
    assert_eq!(contacts[0].phones[0].raw_value, "0812345");
}

#[test]
fn test_parse_mediastore() {
    let device_id = DeviceId::new("test");
    let output = "Row: 0 _data=/sdcard/img.jpg, _size=1024, date_modified=1725000000, mime_type=image/jpeg, width=1920, height=1080, datetaken=1725000000000, latitude=-6.2, longitude=106.8\n";

    let entries = MediaParser::parse_mediastore(&device_id, output);
    assert_eq!(entries.len(), 1);
    let f = &entries[0];
    assert_eq!(f.path, "/sdcard/img.jpg");
    assert_eq!(f.size_bytes, 1024);

    let info = f.media_info.as_ref().unwrap();
    assert_eq!(info.width, Some(1920));
    assert_eq!(info.latitude, Some(-6.2));
}

#[test]
fn test_parse_battery() {
    let output = "Current Battery Service state:\n\
                    AC powered: false\n\
                    USB powered: true\n\
                    Wireless powered: false\n\
                    Max charging current: 500000\n\
                    Max charging voltage: 5000000\n\
                    Charge counter: 4500000\n\
                    status: 2\n\
                    health: 2\n\
                    present: true\n\
                    level: 85\n\
                    scale: 100\n\
                    voltage: 4100\n\
                    temperature: 325\n\
                    technology: Li-ion";

    let status =
        phone_backup_adapter_adb::parsers::battery_parser::BatteryParser::parse(output).unwrap();
    assert_eq!(status.level, 85);
    assert!(status.is_charging);
    assert_eq!(status.temperature, 32.5);
}
