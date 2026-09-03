use chrono::{TimeZone, Utc};
use phone_backup_whatsapp::{
    ChatType, WhatsAppChatBuilder, WhatsAppExportFactory, WhatsAppExportFormat,
};

#[test]
fn test_whatsapp_export_factory_html_and_json() {
    let t1 = Utc.with_ymd_and_hms(2026, 3, 1, 9, 30, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2026, 3, 1, 9, 31, 0).unwrap();

    let chat = WhatsAppChatBuilder::new("6281234567890@s.whatsapp.net", ChatType::Individual)
        .with_name("Budi Santoso")
        .add_text_message("msg1", "6281234567890@s.whatsapp.net", false, t1, "Halo! Apa kabar?")
        .add_text_message("msg2", "me", true, t2, "Kabar baik bro!")
        .build();

    let chats = vec![chat];

    // 1. HTML Viewer Export
    let html = WhatsAppExportFactory::export(&chats, WhatsAppExportFormat::Html)
        .expect("HTML export failed");
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("WhatsApp Offline Archive"));
    assert!(html.contains("Budi Santoso"));
    assert!(html.contains("Halo! Apa kabar?"));
    assert!(html.contains("from-me"));
    assert!(html.contains("from-other"));

    // 2. JSON Export
    let json = WhatsAppExportFactory::export(&chats, WhatsAppExportFormat::Json)
        .expect("JSON export failed");
    assert!(json.contains("6281234567890@s.whatsapp.net"));
    assert!(json.contains("Kabar baik bro!"));
}
