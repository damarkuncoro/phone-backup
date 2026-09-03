use phone_backup_contacts::{ContactBook, ContactBuilder, PhoneType};

#[test]
fn test_contact_diff_engine_lifecycle() {
    let snapshot_v1 = ContactBook::builder()
        .with_default_country_code("+62")
        .add_contact(
            ContactBuilder::new("Alice Wonderland")
                .add_phone("+628111111111", PhoneType::Mobile)
                .build(),
        )
        .add_contact(
            ContactBuilder::new("Bob Builder")
                .add_phone("+628222222222", PhoneType::Work)
                .build(),
        )
        .build();

    let snapshot_v2 = ContactBook::builder()
        .with_default_country_code("+62")
        // Alice was modified (added new phone)
        .add_contact(
            ContactBuilder::new("Alice Wonderland")
                .add_phone("+628111111111", PhoneType::Mobile)
                .add_phone("+628111111112", PhoneType::Home)
                .build(),
        )
        // Bob was deleted (not present in v2)
        // Charlie was newly added
        .add_contact(
            ContactBuilder::new("Charlie Chaplin")
                .add_phone("+628333333333", PhoneType::Mobile)
                .build(),
        )
        .build();

    let diff = snapshot_v1.diff(&snapshot_v2);

    assert_eq!(diff.added.len(), 1);
    assert_eq!(diff.added[0].display_name, "Charlie Chaplin");

    assert_eq!(diff.modified.len(), 1);
    assert_eq!(diff.modified[0].1.display_name, "Alice Wonderland");
    assert_eq!(diff.modified[0].1.phone_numbers.len(), 2);

    assert_eq!(diff.deleted.len(), 1);
    assert_eq!(diff.deleted[0].display_name, "Bob Builder");
}
