use phone_backup_messages::{CallEntryBuilder, CallLogAnalytics, CallType};

#[test]
fn test_call_statistics_summary() {
    let call1 = CallEntryBuilder::new("1", "+628111111111")
        .with_duration(120)
        .with_type(CallType::Incoming)
        .build();

    let call2 = CallEntryBuilder::new("2", "+628111111111")
        .with_duration(180)
        .with_type(CallType::Outgoing)
        .build();

    let call3 = CallEntryBuilder::new("3", "+628222222222")
        .with_duration(0)
        .with_type(CallType::Missed)
        .build();

    let summary = CallLogAnalytics::compute_summary(&[call1, call2, call3]);

    assert_eq!(summary.total_calls, 3);
    assert_eq!(summary.incoming_count, 1);
    assert_eq!(summary.outgoing_count, 1);
    assert_eq!(summary.missed_count, 1);
    assert_eq!(summary.total_duration_seconds, 300);
    assert_eq!(summary.average_duration_seconds, 150);
    assert_eq!(summary.top_numbers[0].0, "+628111111111");
    assert_eq!(summary.top_numbers[0].1, 2);
}
