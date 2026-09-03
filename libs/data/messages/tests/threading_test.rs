use chrono::{TimeZone, Utc};
use phone_backup_messages::{MessageType, SmsMessageBuilder, ThreadEngine};

#[test]
fn test_thread_reconstruction_and_sorting() {
    let t1 = Utc.with_ymd_and_hms(2026, 3, 1, 10, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2026, 3, 1, 10, 5, 0).unwrap();
    let t3 = Utc.with_ymd_and_hms(2026, 3, 1, 11, 0, 0).unwrap();

    let msg1 = SmsMessageBuilder::new("1", "+6281211112222", "Pagi!")
        .with_date(t1)
        .with_type(MessageType::Inbox)
        .with_contact_name("Budi")
        .build();

    let msg2 = SmsMessageBuilder::new("2", "+6281211112222", "Pagi juga bro")
        .with_date(t2)
        .with_type(MessageType::Sent)
        .with_contact_name("Budi")
        .build();

    let msg3 = SmsMessageBuilder::new("3", "+6289999999999", "Meeting jam 11 ya")
        .with_date(t3)
        .with_type(MessageType::Inbox)
        .with_contact_name("Siti")
        .build();

    let threads = ThreadEngine::build_threads(vec![msg2, msg1, msg3]);

    assert_eq!(threads.len(), 2);
    // Thread with Siti has newer message (11:00) so it comes first
    assert_eq!(threads[0].participant.contact_name.as_deref(), Some("Siti"));
    assert_eq!(threads[1].participant.contact_name.as_deref(), Some("Budi"));

    // Messages in Budi's thread must be ordered chronologically (10:00 then 10:05)
    assert_eq!(threads[1].messages.len(), 2);
    assert_eq!(threads[1].messages[0].body, "Pagi!");
    assert_eq!(threads[1].messages[1].body, "Pagi juga bro");
}
