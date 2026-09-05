use crate::domain::CalendarEvent;

/// Serializer for generating RFC 5545 `.ics` files.
pub struct IcsExporter;

impl IcsExporter {
    /// Serializes a slice of `CalendarEvent` into RFC 5545 iCalendar format.
    pub fn export(events: &[CalendarEvent]) -> String {
        let mut out = String::new();
        out.push_str("BEGIN:VCALENDAR\r\n");
        out.push_str("VERSION:2.0\r\n");
        out.push_str("PRODID:-//Damar Kuncoro//Phone Backup Calendar//EN\r\n");

        for event in events {
            out.push_str("BEGIN:VEVENT\r\n");
            out.push_str(&format!("UID:{}\r\n", event.id));
            out.push_str(&format!("SUMMARY:{}\r\n", escape_ics(&event.summary)));

            if let Some(ref desc) = event.description {
                out.push_str(&format!("DESCRIPTION:{}\r\n", escape_ics(desc)));
            }
            if let Some(ref loc) = event.location {
                out.push_str(&format!("LOCATION:{}\r\n", escape_ics(loc)));
            }

            if event.is_all_day {
                out.push_str(&format!("DTSTART;VALUE=DATE:{}\r\n", event.start_time.format("%Y%m%d")));
                out.push_str(&format!("DTEND;VALUE=DATE:{}\r\n", event.end_time.format("%Y%m%d")));
            } else {
                out.push_str(&format!("DTSTART:{}\r\n", event.start_time.format("%Y%m%dT%H%M%SZ")));
                out.push_str(&format!("DTEND:{}\r\n", event.end_time.format("%Y%m%dT%H%M%SZ")));
            }

            if let Some(ref rrule) = event.recurrence {
                let freq_str = match rrule.frequency {
                    crate::domain::Frequency::Daily => "DAILY",
                    crate::domain::Frequency::Weekly => "WEEKLY",
                    crate::domain::Frequency::Monthly => "MONTHLY",
                    crate::domain::Frequency::Yearly => "YEARLY",
                };
                if rrule.interval > 1 {
                    out.push_str(&format!("RRULE:FREQ={};INTERVAL={}\r\n", freq_str, rrule.interval));
                } else {
                    out.push_str(&format!("RRULE:FREQ={}\r\n", freq_str));
                }
            }

            if !event.categories.is_empty() {
                out.push_str(&format!("CATEGORIES:{}\r\n", event.categories.join(",")));
            }

            out.push_str("END:VEVENT\r\n");
        }

        out.push_str("END:VCALENDAR\r\n");
        out
    }
}

fn escape_ics(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace('\n', "\\n")
}
