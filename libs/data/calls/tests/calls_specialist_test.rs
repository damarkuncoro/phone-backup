use chrono::Utc;
use phone_backup_calls::{
    CallAnalytics, CallLogFactory, CallLogItemBuilder, CallType, CsvCallExporter, JsonCallExporter,
    JsonCallParser, XmlCallParser,
};

#[test]
fn test_call_type_mappings() {
    assert_eq!(CallType::from_android_type(1), CallType::Incoming);
    assert_eq!(CallType::from_android_type(2), CallType::Outgoing);
    assert_eq!(CallType::from_android_type(3), CallType::Missed);
    assert_eq!(CallType::from_android_type(5), CallType::Rejected);
    assert_eq!(CallType::from_str_loose("missed"), CallType::Missed);
    assert!(CallType::Incoming.is_connected());
    assert!(!CallType::Missed.is_connected());
}

#[test]
fn test_builder_and_factory() {
    let item_from_factory = CallLogFactory::incoming("+628123456789", Some("Alice".into()), 125);
    assert_eq!(item_from_factory.call_type, CallType::Incoming);
    assert_eq!(item_from_factory.duration_display(), "02:05");
    assert_eq!(item_from_factory.caller_label(), "Alice");

    let item_from_builder = CallLogItemBuilder::new()
        .id("custom_id_1")
        .phone_number("+628987654321")
        .contact_name("Bob")
        .call_type(CallType::Missed)
        .timestamp(Utc::now())
        .duration_secs(0)
        .sim_slot(1)
        .build()
        .expect("Build should succeed");

    assert_eq!(item_from_builder.id, "custom_id_1");
    assert_eq!(item_from_builder.call_type, CallType::Missed);
    assert_eq!(item_from_builder.sim_slot, Some(1));
}

#[test]
fn test_xml_call_parser() {
    let xml = r#"<?xml version='1.0' encoding='UTF-8' standalone='yes' ?>
<calls count="3">
  <call number="+6281111111" duration="180" date="1725450000000" type="1" contact_name="Charlie" />
  <call number="+6282222222" duration="45" date="1725451000000" type="2" contact_name="Dave" />
  <call number="+6283333333" duration="0" date="1725452000000" type="3" contact_name="(Unknown)" />
</calls>"#;

    let items = XmlCallParser::parse(xml);
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].phone_number, "+6281111111");
    assert_eq!(items[0].call_type, CallType::Incoming);
    assert_eq!(items[0].duration_secs, 180);
    assert_eq!(items[0].contact_name.as_deref(), Some("Charlie"));

    assert_eq!(items[1].call_type, CallType::Outgoing);
    assert_eq!(items[2].call_type, CallType::Missed);
    assert_eq!(items[2].contact_name, None);
}

#[test]
fn test_analytics_and_stats() {
    let call1 = CallLogFactory::incoming("+6281111111", Some("Alice".into()), 200);
    let call2 = CallLogFactory::incoming("+6281111111", Some("Alice".into()), 100);
    let call3 = CallLogFactory::outgoing("+6282222222", Some("Bob".into()), 50);
    let call4 = CallLogFactory::missed("+6283333333", None);

    let stats = CallAnalytics::compute_stats(&[call1, call2, call3, call4]);
    assert_eq!(stats.total_calls, 4);
    assert_eq!(stats.incoming_count, 2);
    assert_eq!(stats.outgoing_count, 1);
    assert_eq!(stats.missed_count, 1);
    assert_eq!(stats.total_duration_secs, 350);
    assert_eq!(stats.frequent_contacts[0].phone_number, "+6281111111");
    assert_eq!(stats.frequent_contacts[0].call_count, 2);
    assert_eq!(stats.frequent_contacts[0].total_duration_secs, 300);
}

#[test]
fn test_exporters_and_parsers_roundtrip() {
    let call1 = CallLogFactory::incoming("+6281111111", Some("Alice".into()), 120);
    let csv = CsvCallExporter::export(&[call1.clone()]);
    assert!(csv.contains("+6281111111"));
    assert!(csv.contains("Alice"));

    let json = JsonCallExporter::export_pretty(&[call1]).expect("JSON export should succeed");
    let parsed_json = JsonCallParser::parse(&json);
    assert_eq!(parsed_json.len(), 1);
    assert_eq!(parsed_json[0].phone_number, "+6281111111");
}
