use phone_backup_contacts::{ContactBook, ContactBuilder, EmailType, ExportFormat, PhoneType};

#[test]
fn test_formatter_factory_exports() {
    let book = ContactBook::builder()
        .add_contact(
            ContactBuilder::new("Dewi Sartika")
                .add_phone("+628123456789", PhoneType::Mobile)
                .add_email("dewi@example.org", EmailType::Personal)
                .with_organization("Education Hero", None)
                .build(),
        )
        .build();

    // 1. CSV Format
    let csv = book.export(ExportFormat::Csv).unwrap();
    assert!(csv.starts_with("Name,Given Name,Family Name"));
    assert!(csv.contains("Dewi Sartika"));
    assert!(csv.contains("+628123456789"));

    // 2. JSON Format
    let json = book.export(ExportFormat::Json).unwrap();
    assert!(json.contains("\"display_name\": \"Dewi Sartika\""));

    // 3. NDJSON Format
    let ndjson = book.export(ExportFormat::Ndjson).unwrap();
    assert!(ndjson.lines().count() == 1);
    assert!(ndjson.contains("\"display_name\":\"Dewi Sartika\""));

    // 4. vCard 4.0 Format
    let vcard4 = book.export(ExportFormat::VCard4).unwrap();
    assert!(vcard4.contains("VERSION:4.0"));
    assert!(vcard4.contains("FN:Dewi Sartika"));
}
