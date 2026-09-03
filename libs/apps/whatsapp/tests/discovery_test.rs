use phone_backup_whatsapp::{MediaCategory, WhatsAppPathScanner};

#[test]
fn test_whatsapp_scoped_and_legacy_paths() {
    let roots = WhatsAppPathScanner::candidate_roots();
    assert!(roots.len() >= 3);
    assert_eq!(roots[0], WhatsAppPathScanner::SCOPED_STORAGE_BASE);
    assert_eq!(roots[1], WhatsAppPathScanner::SCOPED_BUSINESS_BASE);
    assert_eq!(roots[2], WhatsAppPathScanner::LEGACY_STORAGE_BASE);
}

#[test]
fn test_whatsapp_media_path_categorization() {
    let cat_ptt = WhatsAppPathScanner::categorize_path("WhatsApp/Media/WhatsApp Voice Notes/202328/PTT-20230715-WA0000.opus");
    assert_eq!(cat_ptt, MediaCategory::VoiceNotes);

    let cat_img = WhatsAppPathScanner::categorize_path("Android/media/com.whatsapp/WhatsApp/Media/WhatsApp Images/Sent/IMG-20230715-WA0000(1).jpg");
    assert_eq!(cat_img, MediaCategory::Images);

    let cat_vid = WhatsAppPathScanner::categorize_path("WhatsApp/Media/WhatsApp Video/VID-20230715-WA0001.mp4");
    assert_eq!(cat_vid, MediaCategory::Video);

    let cat_doc = WhatsAppPathScanner::categorize_path("WhatsApp/Media/WhatsApp Documents/Laporan.pdf");
    assert_eq!(cat_doc, MediaCategory::Documents);
}
