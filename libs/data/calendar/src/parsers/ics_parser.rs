use crate::domain::{CalendarEvent, Frequency, RecurrenceRule};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

/// Parser for standard RFC 5545 iCalendar (`.ics`) files.
pub struct IcsParser;

impl IcsParser {
    /// Parses an iCalendar string into a vector of `CalendarEvent`.
    pub fn parse(ics_content: &str) -> Vec<CalendarEvent> {
        let mut events = Vec::new();
        let mut in_event = false;

        let mut uid = String::new();
        let mut summary = String::new();
        let mut description = None;
        let mut location = None;
        let mut start_time = None;
        let mut end_time = None;
        let mut is_all_day = false;
        let mut recurrence = None;
        let mut categories = Vec::new();

        for raw_line in ics_content.lines() {
            let line = raw_line.trim();
            if line == "BEGIN:VEVENT" {
                in_event = true;
                uid.clear();
                summary.clear();
                description = None;
                location = None;
                start_time = None;
                end_time = None;
                is_all_day = false;
                recurrence = None;
                categories.clear();
                continue;
            }

            if line == "END:VEVENT" {
                if in_event {
                    let st = start_time.unwrap_or_else(Utc::now);
                    let et = end_time.unwrap_or(st);
                    let id = if uid.is_empty() {
                        format!("event_{}", events.len() + 1)
                    } else {
                        uid.clone()
                    };

                    let mut event = CalendarEvent::new(id, summary.clone(), st, et);
                    event.description = description.clone();
                    event.location = location.clone();
                    event.is_all_day = is_all_day;
                    event.recurrence = recurrence.clone();
                    event.categories = categories.clone();
                    events.push(event);
                }
                in_event = false;
                continue;
            }

            if !in_event {
                continue;
            }

            if let Some(val) = strip_prefix_case(line, "UID:") {
                uid = val.to_string();
            } else if let Some(val) = strip_prefix_case(line, "SUMMARY:") {
                summary = val.to_string();
            } else if let Some(val) = strip_prefix_case(line, "DESCRIPTION:") {
                description = Some(val.to_string());
            } else if let Some(val) = strip_prefix_case(line, "LOCATION:") {
                location = Some(val.to_string());
            } else if line.starts_with("DTSTART") {
                let (dt, all_day) = parse_ics_date(line);
                start_time = dt;
                if all_day { is_all_day = true; }
            } else if line.starts_with("DTEND") {
                let (dt, _) = parse_ics_date(line);
                end_time = dt;
            } else if let Some(val) = strip_prefix_case(line, "RRULE:") {
                recurrence = parse_rrule(val);
            } else if let Some(val) = strip_prefix_case(line, "CATEGORIES:") {
                categories = val.split(',').map(|s| s.trim().to_string()).collect();
            }
        }

        events
    }
}

fn strip_prefix_case<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.len() >= prefix.len() && s[..prefix.len()].eq_ignore_ascii_case(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

fn parse_ics_date(line: &str) -> (Option<DateTime<Utc>>, bool) {
    let val = match line.split(':').next_back() {
        Some(v) => v.trim(),
        None => return (None, false),
    };

    if val.len() == 8 {
        // YYYYMMDD
        if let Ok(d) = NaiveDate::parse_from_str(val, "%Y%m%d") {
            let ndt = d.and_hms_opt(0, 0, 0).unwrap();
            return (Some(DateTime::from_naive_utc_and_offset(ndt, Utc)), true);
        }
    }

    let clean_val = val.trim_end_matches('Z');
    if let Ok(ndt) = NaiveDateTime::parse_from_str(clean_val, "%Y%m%dT%H%M%S") {
        return (Some(DateTime::from_naive_utc_and_offset(ndt, Utc)), false);
    }

    (None, false)
}

fn parse_rrule(rrule_str: &str) -> Option<RecurrenceRule> {
    let mut freq = None;
    let mut interval = 1u32;
    let mut count = None;

    for part in rrule_str.split(';') {
        let mut kv = part.split('=');
        let k = kv.next()?.trim().to_uppercase();
        let v = kv.next()?.trim();

        match k.as_str() {
            "FREQ" => freq = Frequency::from_rrule_str(v),
            "INTERVAL" => interval = v.parse().unwrap_or(1),
            "COUNT" => count = v.parse().ok(),
            _ => {}
        }
    }

    freq.map(|f| {
        let mut r = RecurrenceRule::new(f).with_interval(interval);
        if let Some(c) = count {
            r = r.with_count(c);
        }
        r
    })
}
