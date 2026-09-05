use chrono::{Duration, Utc};
use phone_backup_calendar::{
    CalendarAnalytics, CalendarEventBuilder, CalendarFactory, Frequency, IcsExporter, IcsParser,
    JsonCalendarExporter, JsonCalendarParser, RecurrenceRule,
};

#[test]
fn test_recurrence_rule() {
    let rule = RecurrenceRule::new(Frequency::Weekly).with_interval(2);
    assert_eq!(rule.format_description(), "Repeats every 2 Weekly");
}

#[test]
fn test_builder_and_factory() {
    let now = Utc::now();
    let meeting = CalendarFactory::create_meeting(
        "Quarterly Planning",
        now,
        60,
        "boss@company.com",
        "dev@company.com",
    );
    assert_eq!(meeting.summary, "Quarterly Planning");
    assert_eq!(meeting.duration_minutes(), 60);
    assert_eq!(meeting.organizer.as_ref().unwrap().email, "boss@company.com");
    assert_eq!(meeting.attendees.len(), 1);

    let event_from_builder = CalendarEventBuilder::new()
        .id("evt_100")
        .summary("Sprint Demo")
        .start_time(now)
        .end_time(now + Duration::minutes(45))
        .location("Conference Room A")
        .add_category("Work")
        .build()
        .expect("Build should succeed");

    assert_eq!(event_from_builder.id, "evt_100");
    assert_eq!(event_from_builder.location.as_deref(), Some("Conference Room A"));
    assert_eq!(event_from_builder.categories, vec!["Work"]);
}

#[test]
fn test_ics_parser() {
    let ics = r#"BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//Test//EN
BEGIN:VEVENT
UID:uid_123@google.com
SUMMARY:Team Standup
DESCRIPTION:Daily agile standup
LOCATION:Zoom
DTSTART:20260905T090000Z
DTEND:20260905T093000Z
RRULE:FREQ=DAILY;INTERVAL=1
CATEGORIES:Meetings,Agile
END:VEVENT
END:VCALENDAR"#;

    let events = IcsParser::parse(ics);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, "uid_123@google.com");
    assert_eq!(events[0].summary, "Team Standup");
    assert_eq!(events[0].location.as_deref(), Some("Zoom"));
    assert_eq!(events[0].duration_minutes(), 30);
    assert!(events[0].recurrence.is_some());
    assert_eq!(events[0].recurrence.as_ref().unwrap().frequency, Frequency::Daily);
    assert_eq!(events[0].categories, vec!["Meetings", "Agile"]);
}

#[test]
fn test_conflict_detection_and_analytics() {
    let now = Utc::now();
    let evt1 = CalendarFactory::create_event("Event A", now, 60); // 0..60 min
    let evt2 = CalendarFactory::create_event("Event B", now + Duration::minutes(30), 60); // 30..90 min (overlaps!)
    let evt3 = CalendarFactory::create_event("Event C", now + Duration::minutes(120), 30); // 120..150 min (no overlap)

    let all_events = [evt1.clone(), evt2.clone(), evt3.clone()];
    let conflicts = CalendarAnalytics::find_conflicts(&all_events);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].0.summary, "Event A");
    assert_eq!(conflicts[0].1.summary, "Event B");

    let stats = CalendarAnalytics::compute_stats(&[evt1, evt2, evt3]);
    assert_eq!(stats.total_events, 3);
    assert_eq!(stats.upcoming_count, 3);
}

#[test]
fn test_exporters_and_parsers_roundtrip() {
    let now = Utc::now();
    let evt = CalendarFactory::create_event("Product Launch", now, 120);
    let ics = IcsExporter::export(&[evt.clone()]);
    assert!(ics.contains("BEGIN:VCALENDAR"));
    assert!(ics.contains("Product Launch"));

    let json = JsonCalendarExporter::export_pretty(&[evt]).expect("JSON export should succeed");
    let parsed_json = JsonCalendarParser::parse(&json);
    assert_eq!(parsed_json.len(), 1);
    assert_eq!(parsed_json[0].summary, "Product Launch");
}
