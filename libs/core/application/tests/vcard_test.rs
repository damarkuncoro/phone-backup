use domain::{
    Contact, ContactAddress, ContactEmail, ContactName, ContactOrganization, ContactPhone,
};
use phone_backup_application::VCardEngine;

#[test]
fn test_vcard_export_and_import_roundtrip() {
    let contact = Contact {
        id: "c1".to_string(),
        snapshot_id: None,
        source_id: Some("src123".to_string()),
        display_name: "Damar Kuncoro".to_string(),
        notes: Some("Software Developer".to_string()),
        source: "google".to_string(),
        source_account: Some("damar@example.com".to_string()),
        content_hash: None,
        metadata_json: None,
        names: vec![ContactName {
            display_name: Some("Damar Kuncoro".to_string()),
            given_name: Some("Damar".to_string()),
            middle_name: None,
            family_name: Some("Kuncoro".to_string()),
            prefix: None,
            suffix: None,
        }],
        phones: vec![ContactPhone {
            raw_value: "+6281212345678".to_string(),
            normalized_value: Some("+6281212345678".to_string()),
            phone_type: Some("mobile".to_string()),
            label: None,
            is_primary: true,
        }],
        emails: vec![ContactEmail {
            value: "damar@example.com".to_string(),
            email_type: Some("home".to_string()),
            label: None,
            is_primary: true,
        }],
        addresses: vec![ContactAddress {
            formatted_address: Some("Jakarta, Indonesia".to_string()),
            street: Some("Sudirman".to_string()),
            city: Some("Jakarta".to_string()),
            region: Some("DKI".to_string()),
            postal_code: Some("12190".to_string()),
            country: Some("Indonesia".to_string()),
            country_code: None,
            address_type: Some("work".to_string()),
            label: None,
        }],
        organizations: vec![ContactOrganization {
            company_name: Some("Antigravity Inc".to_string()),
            department: None,
            title: Some("Senior Developer".to_string()),
            job_description: None,
            org_type: None,
            label: None,
        }],
        urls: vec![],
        events: vec![],
        photos: vec![],
        labels: vec!["Work".to_string()],
    };

    // 1. Export to vCard string
    let vcard_str = VCardEngine::export_to_vcard(&[contact]);
    assert!(vcard_str.contains("BEGIN:VCARD"));
    assert!(vcard_str.contains("FN:Damar Kuncoro"));
    assert!(vcard_str.contains("N:Kuncoro;Damar;;;"));
    assert!(vcard_str.contains("TEL;TYPE=MOBILE:+6281212345678"));
    assert!(vcard_str.contains("EMAIL;TYPE=HOME:damar@example.com"));
    assert!(vcard_str.contains("ORG:Antigravity Inc"));
    assert!(vcard_str.contains("END:VCARD"));

    // 2. Import back from vCard string
    let imported = VCardEngine::import_from_vcard(&vcard_str).expect("Failed to parse vCard");
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].display_name, "Damar Kuncoro");
    assert_eq!(imported[0].phones.len(), 1);
    assert_eq!(imported[0].phones[0].raw_value, "+6281212345678");
    assert_eq!(imported[0].emails.len(), 1);
    assert_eq!(imported[0].emails[0].value, "damar@example.com");
}
