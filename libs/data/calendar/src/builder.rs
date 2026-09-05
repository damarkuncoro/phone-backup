use crate::domain::{Attendee, CalendarEvent, Organizer, RecurrenceRule};
use chrono::{DateTime, Duration, Utc};

/// Fluent builder for constructing `CalendarEvent` instances.
#[derive(Default)]
pub struct CalendarEventBuilder {
    id: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    location: Option<String>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    is_all_day: Option<bool>,
    recurrence: Option<RecurrenceRule>,
    organizer: Option<Organizer>,
    attendees: Vec<Attendee>,
    categories: Vec<String>,
}

impl CalendarEventBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn start_time(mut self, start: DateTime<Utc>) -> Self {
        self.start_time = Some(start);
        self
    }

    pub fn end_time(mut self, end: DateTime<Utc>) -> Self {
        self.end_time = Some(end);
        self
    }

    pub fn is_all_day(mut self, all_day: bool) -> Self {
        self.is_all_day = Some(all_day);
        self
    }

    pub fn recurrence(mut self, recurrence: RecurrenceRule) -> Self {
        self.recurrence = Some(recurrence);
        self
    }

    pub fn organizer(mut self, organizer: Organizer) -> Self {
        self.organizer = Some(organizer);
        self
    }

    pub fn add_attendee(mut self, attendee: Attendee) -> Self {
        self.attendees.push(attendee);
        self
    }

    pub fn add_category(mut self, category: impl Into<String>) -> Self {
        self.categories.push(category.into());
        self
    }

    pub fn build(self) -> Result<CalendarEvent, &'static str> {
        let summary = self.summary.ok_or("Event summary is required")?;
        let start_time = self.start_time.unwrap_or_else(Utc::now);
        let end_time = self.end_time.unwrap_or_else(|| start_time + Duration::hours(1));
        let id = self.id.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            format!("event_{}", nanos)
        });

        let mut event = CalendarEvent::new(id, summary, start_time, end_time);
        event.description = self.description;
        event.location = self.location;
        event.is_all_day = self.is_all_day.unwrap_or(false);
        event.recurrence = self.recurrence;
        event.organizer = self.organizer;
        event.attendees = self.attendees;
        event.categories = self.categories;

        Ok(event)
    }
}
