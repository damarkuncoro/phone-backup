use crate::domain::{CalendarEvent, RecurrenceRule};
use chrono::{DateTime, Utc};

/// Parser for Android Calendar Content Provider query outputs.
pub struct AndroidCalendarParser;

impl AndroidCalendarParser {
    fn extract_value(line: &str, key: &str) -> Option<String> {
        let key_with_eq = format!("{}=", key);
        if let Some(start) = line.find(&key_with_eq) {
            let value_part = &line[start + key_with_eq.len()..];
            let value = if let Some(end) = value_part.find(", ") {
                value_part[..end].trim().to_string()
            } else {
                value_part.trim().to_string()
            };

            if value.eq_ignore_ascii_case("null") || value.is_empty() {
                return None;
            }
            return Some(value);
        }
        None
    }

    /// Parses output lines from Android calendar content query.
    pub fn parse(output: &str) -> Vec<CalendarEvent> {
        let mut events = Vec::new();

        for line in output.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("Row:") {
                continue;
            }

            let summary = Self::extract_value(trimmed, "title")
                .unwrap_or_else(|| "Untitled Event".to_string());

            let start_ms: i64 = Self::extract_value(trimmed, "dtstart")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let start_time = DateTime::from_timestamp_millis(start_ms).unwrap_or_else(Utc::now);

            let end_ms: i64 = Self::extract_value(trimmed, "dtend")
                .and_then(|s| s.parse().ok())
                .unwrap_or(start_ms + 3600 * 1000);
            let end_time = DateTime::from_timestamp_millis(end_ms).unwrap_or(start_time);

            let is_all_day = Self::extract_value(trimmed, "allDay")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);

            let location = Self::extract_value(trimmed, "eventLocation");
            let description = Self::extract_value(trimmed, "description");
            let rrule = Self::extract_value(trimmed, "rrule").and_then(|r| RecurrenceRule::parse(&r));

            let id = format!(
                "event_{}_{}",
                start_ms,
                summary.chars().take(10).collect::<String>().replace(' ', "_")
            );

            let mut event = CalendarEvent::new(id, summary, start_time, end_time);
            event.is_all_day = is_all_day;
            event.location = location;
            event.description = description;
            event.recurrence = rrule;

            events.push(event);
        }

        events
    }
}
