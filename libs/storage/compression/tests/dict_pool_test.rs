use phone_backup_compression::dict::{DictionaryId, DictionaryManager};

#[test]
fn test_categorized_dictionaries_roundtrip() {
    let manager = DictionaryManager::with_android_defaults();

    // 1. Android XML sample
    let xml_sample = b"<manifest xmlns:android=\"http://schemas.android.com/apk/res/android\" package=\"com.example.app\"><application android:name=\".App\"><activity android:name=\".MainActivity\" android:exported=\"true\"/></application></manifest>";
    let xml_dict_id = DictionaryId::new("android-xml-v1");
    let compressed_xml = manager.compress_with_dict(xml_sample, &xml_dict_id, 3).expect("XML compression failed");
    let decompressed_xml = manager.decompress_with_dict(&compressed_xml, &xml_dict_id).expect("XML decompression failed");
    assert_eq!(decompressed_xml, xml_sample);

    // 2. vCard Contacts sample
    let vcard_sample = b"BEGIN:VCARD\r\nVERSION:4.0\r\nFN:John Doe\r\nTEL;TYPE=CELL:+1234567890\r\nEMAIL;TYPE=HOME:john@example.com\r\nEND:VCARD\r\n";
    let vcard_dict_id = DictionaryId::new("android-vcard-v1");
    let compressed_vcard = manager.compress_with_dict(vcard_sample, &vcard_dict_id, 3).expect("vCard compression failed");
    let decompressed_vcard = manager.decompress_with_dict(&compressed_vcard, &vcard_dict_id).expect("vCard decompression failed");
    assert_eq!(decompressed_vcard, vcard_sample);

    // 3. WhatsApp Chat Export sample
    let wa_sample = b"Messages and calls are end-to-end encrypted. No one outside of this chat can read or listen to them.\n[12/03/26, 10:00:00] Alice: Halo apa kabar?\n<Media omitted>\nThis message was deleted\n";
    let wa_dict_id = DictionaryId::new("android-whatsapp-v1");
    let compressed_wa = manager.compress_with_dict(wa_sample, &wa_dict_id, 3).expect("WhatsApp compression failed");
    let decompressed_wa = manager.decompress_with_dict(&compressed_wa, &wa_dict_id).expect("WhatsApp decompression failed");
    assert_eq!(decompressed_wa, wa_sample);
}
