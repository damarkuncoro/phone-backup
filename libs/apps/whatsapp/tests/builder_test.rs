use phone_backup_whatsapp::{ChatType, WhatsAppBackupStore, WhatsAppChatBuilder};

#[test]
fn test_whatsapp_backup_store_lifecycle() {
    let mut store = WhatsAppBackupStore::new();

    let chat = WhatsAppChatBuilder::new("group123@g.us", ChatType::Group)
        .with_name("Developer Team")
        .build();

    store.add_chat(chat);
    store.index_media_file("WhatsApp/Media/WhatsApp Voice Notes/202328/PTT-20230715-WA0000.opus", 50_000);
    store.index_media_file("WhatsApp/Media/WhatsApp Images/IMG-20230715-WA0001.jpg", 120_000);

    assert_eq!(store.chats.len(), 1);
    assert_eq!(store.media_items.len(), 2);

    let summary = store.media_summary();
    assert_eq!(summary.total_files, 2);
    assert_eq!(summary.total_bytes, 170_000);
}
