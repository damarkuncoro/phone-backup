use phone_backup_contacts::{ContactBook, ContactBuilder, EmailType, PhoneType};

#[test]
fn test_contact_deduplication_and_lossless_merge() {
    let mut book = ContactBook::builder()
        .with_default_country_code("+62")
        // Contact 1 (From SIM Card - only name & raw mobile)
        .add_contact(
            ContactBuilder::new("Budi Santoso")
                .add_phone("081234567890", PhoneType::Mobile)
                .build(),
        )
        // Contact 2 (From Google Account - with email & company & normalized phone)
        .add_contact(
            ContactBuilder::new("Budi Santoso")
                .add_phone("+6281234567890", PhoneType::Mobile)
                .add_email("budi.santoso@example.com", EmailType::Work)
                .with_organization("Tech Corp", Some("Senior Engineer"))
                .build(),
        )
        // Contact 3 (Distinct Contact)
        .add_contact(
            ContactBuilder::new("Siti Aminah")
                .add_phone("089876543210", PhoneType::Mobile)
                .build(),
        )
        .build();

    assert_eq!(book.len(), 3);

    // Run smart deduplication
    book.deduplicate();

    // Now Budi Santoso should be merged into 1 consolidated contact
    assert_eq!(book.len(), 2);

    let budi = book.contacts.iter().find(|c| c.display_name == "Budi Santoso").unwrap();
    assert_eq!(budi.phone_numbers.len(), 1);
    assert_eq!(budi.emails.len(), 1);
    assert_eq!(budi.emails[0].email, "budi.santoso@example.com");
    assert!(budi.organization.is_some());
}
