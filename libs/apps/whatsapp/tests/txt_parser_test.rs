use phone_backup_whatsapp::WhatsAppTxtParser;
use phone_backup_whatsapp::{WhatsAppExportFactory, WhatsAppExportFormat};

#[test]
fn test_parse_standard_android_exported_chat() {
    let raw_chat = "\
12/08/2026, 09:15 - Messages and calls are end-to-end encrypted. No one outside of this chat, not even WhatsApp, can read or listen to them.
12/08/2026, 09:16 - Budi Pratama: Halo mas, file backup sudah siap?
12/08/2026, 09:17 - Anda: Sudah siap mas.
Ini rinciannya:
1. Kontak: 299
2. SMS: 8900 baris
3. Call Logs: 1430
12/08/2026, 09:18 - Budi Pratama: Mantap sekali, terima kasih!
";

    let chat = WhatsAppTxtParser::parse("Budi Pratama", raw_chat).expect("Failed to parse chat");

    assert_eq!(chat.name.as_deref(), Some("Budi Pratama"));
    assert_eq!(chat.messages.len(), 3);

    // First user message
    assert_eq!(chat.messages[0].sender_jid, "Budi Pratama");
    assert!(!chat.messages[0].from_me);
    assert_eq!(chat.messages[0].body, "Halo mas, file backup sudah siap?");

    // Second message (multi-line from "Anda")
    assert_eq!(chat.messages[1].sender_jid, "Anda");
    assert!(chat.messages[1].from_me);
    assert!(chat.messages[1].body.contains("Ini rinciannya:\n1. Kontak: 299"));

    // Third message
    assert_eq!(chat.messages[2].sender_jid, "Budi Pratama");
    assert_eq!(chat.messages[2].body, "Mantap sekali, terima kasih!");

    // Export to HTML
    let html = WhatsAppExportFactory::export(&[chat], WhatsAppExportFormat::Html).expect("Export failed");
    assert!(html.contains("Budi Pratama"));
    assert!(html.contains("Halo mas, file backup sudah siap?"));
    assert!(html.contains("from-me"));
}

#[test]
fn test_parse_bracket_ios_exported_chat() {
    let raw_chat = "\
[14/07/26, 10.30.00] Siti Rahma: Selamat pagi Pak
[14/07/26, 10.31.25] Anda: Pagi Bu Siti
";

    let chat = WhatsAppTxtParser::parse("Siti Rahma", raw_chat).expect("Failed to parse iOS chat");

    assert_eq!(chat.messages.len(), 2);
    assert_eq!(chat.messages[0].sender_jid, "Siti Rahma");
    assert_eq!(chat.messages[0].body, "Selamat pagi Pak");
    assert_eq!(chat.messages[1].sender_jid, "Anda");
    assert_eq!(chat.messages[1].body, "Pagi Bu Siti");
}
