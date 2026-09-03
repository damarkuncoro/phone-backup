use phone_backup_messages::{
    MessageExportFormat, MessageFormatterFactory, MessageType, SmsMessageBuilder,
};

#[test]
fn test_export_xml_html_csv_json() {
    let msg1 = SmsMessageBuilder::new("1", "+6281234567890", "Halo dunia & teman-teman <testing>")
        .with_type(MessageType::Inbox)
        .with_contact_name("Damar")
        .build();

    let msg2 = SmsMessageBuilder::new("2", "+6281234567890", "Siap!")
        .with_type(MessageType::Sent)
        .with_contact_name("Damar")
        .build();

    let list = vec![msg1, msg2];

    // 1. XML (SMS Backup & Restore compatible)
    let xml = MessageFormatterFactory::export(&list, MessageExportFormat::Xml).expect("XML export failed");
    assert!(xml.contains("<?xml"));
    assert!(xml.contains("<smses count=\"2\">"));
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("&lt;testing&gt;"));

    // 2. HTML
    let html = MessageFormatterFactory::export(&list, MessageExportFormat::Html).expect("HTML export failed");
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Damar"));
    assert!(html.contains("bubble"));

    // 3. CSV
    let csv = MessageFormatterFactory::export(&list, MessageExportFormat::Csv).expect("CSV export failed");
    assert!(csv.contains("ID,Address,ContactName,Date,Type,Read,Body"));
    assert!(csv.contains("INBOX"));
    assert!(csv.contains("SENT"));

    // 4. JSON
    let json = MessageFormatterFactory::export(&list, MessageExportFormat::Json).expect("JSON export failed");
    assert!(json.contains("\"body\": \"Siap!\""));
}
