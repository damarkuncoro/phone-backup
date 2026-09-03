use phone_backup_contacts::{
    ContactBuilder, ContactPhoto, EmailType, PhoneType, StructuredName, VCardParser, VCardVersion,
    VCardWriter,
};

#[test]
fn test_vcard_serialization_and_parsing_roundtrip() {
    let dummy_photo = ContactPhoto {
        mime_type: "image/jpeg".to_string(),
        data: vec![0xFF, 0xD8, 0xFF, 0xE0, 0x01, 0x02, 0x03, 0x04],
        is_primary: true,
    };

    let mut contact = ContactBuilder::new("Dr. Damar Kuncoro, Ph.D.")
        .with_structured_name(
            StructuredName::new()
                .with_prefix("Dr.")
                .with_given("Damar")
                .with_family("Kuncoro")
                .with_suffix("Ph.D."),
        )
        .add_phone("+6281234567890", PhoneType::Mobile)
        .add_phone("+62215551234", PhoneType::Work)
        .add_email("damar@example.com", EmailType::Personal)
        .with_organization("Open Source Foundation", Some("Lead Architect"))
        .with_notes("Key contributor to Android backup systems.")
        .with_birthday("1995-08-17")
        .starred(true)
        .build();
    contact.photos.push(dummy_photo.clone());

    // 1. Serialize to vCard 3.0
    let vcard_str = VCardWriter::write_single(&contact, VCardVersion::V3_0);
    assert!(vcard_str.contains("BEGIN:VCARD"));
    assert!(vcard_str.contains("VERSION:3.0"));
    assert!(vcard_str.contains("FN:Dr. Damar Kuncoro, Ph.D."));
    assert!(vcard_str.contains("PHOTO;ENCODING=b"));
    assert!(vcard_str.contains("END:VCARD"));

    // 2. Parse back from vCard string
    let parsed_contacts = VCardParser::parse_str(&vcard_str).unwrap();
    assert_eq!(parsed_contacts.len(), 1);

    let parsed = &parsed_contacts[0];
    assert_eq!(parsed.display_name, "Dr. Damar Kuncoro, Ph.D.");
    assert_eq!(parsed.phone_numbers.len(), 2);
    assert_eq!(parsed.emails.len(), 1);
    assert_eq!(parsed.emails[0].email, "damar@example.com");
    assert_eq!(parsed.photos.len(), 1);
    assert_eq!(parsed.photos[0].data, dummy_photo.data);
}
