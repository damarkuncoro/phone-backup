use crate::domain::{Attendee, CalendarEvent, Organizer};
use chrono::{DateTime, Duration, Utc};

/// Factory for creating pre-configured `CalendarEvent` instances.
pub struct CalendarFactory;

impl CalendarFactory {
    /// Creates a standard timed event.
    pub fn create_event(
        summary: impl Into<String>,
        start_time: DateTime<Utc>,
        duration_mins: i64,
    ) -> CalendarEvent {
        let end_time = start_time + Duration::minutes(duration_mins);
        let id = format!("evt_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        CalendarEvent::new(id, summary, start_time, end_time)
    }

    /// Creates an all-day event for a given date.
    pub fn create_all_day(summary: impl Into<String>, date: DateTime<Utc>) -> CalendarEvent {
        let id = format!("allday_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let mut event = CalendarEvent::new(id, summary, date, date + Duration::days(1));
        event.is_all_day = true;
        event
    }

    /// Creates a business meeting event with organizer and attendee.
    pub fn create_meeting(
        summary: impl Into<String>,
        start_time: DateTime<Utc>,
        duration_mins: i64,
        organizer_email: impl Into<String>,
        attendee_email: impl Into<String>,
    ) -> CalendarEvent {
        let mut event = Self::create_event(summary, start_time, duration_mins);
        event.organizer = Some(Organizer::new(organizer_email));
        event.attendees.push(Attendee::new(attendee_email));
        event
    }
}
