use phone_backup_whatsapp::{MediaCategory, WhatsAppMediaIndexer};

#[test]
fn test_whatsapp_media_indexing_and_date_extraction() {
    let ptt_path = "WhatsApp/Media/WhatsApp Voice Notes/202328/PTT-20230715-WA0000.opus";
    let item1 = WhatsAppMediaIndexer::parse_media_item(ptt_path, 45_000);

    assert_eq!(item1.category, MediaCategory::VoiceNotes);
    assert!(!item1.is_sent);
    assert!(item1.date_created.is_some());
    assert_eq!(item1.date_created.unwrap().format("%Y-%m-%d").to_string(), "2023-07-15");

    let img_sent_path = "WhatsApp/Media/WhatsApp Images/Sent/IMG-20230715-WA0000(1).jpg";
    let item2 = WhatsAppMediaIndexer::parse_media_item(img_sent_path, 107_483);

    assert_eq!(item2.category, MediaCategory::Images);
    assert!(item2.is_sent);
    assert_eq!(item2.date_created.unwrap().format("%Y-%m-%d").to_string(), "2023-07-15");

    let summary = WhatsAppMediaIndexer::summarize(&[item1, item2]);
    assert_eq!(summary.total_files, 2);
    assert_eq!(summary.total_bytes, 152_483);
    assert_eq!(summary.voice_notes_count, 1);
    assert_eq!(summary.images_count, 1);
    assert_eq!(summary.sent_files_count, 1);
}
