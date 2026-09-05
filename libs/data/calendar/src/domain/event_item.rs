use super::attendee::{Attendee, Organizer};
use super::recurrence::RecurrenceRule;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Aggregate root representing an individual calendar event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_all_day: bool,
    pub recurrence: Option<RecurrenceRule>,
    pub organizer: Option<Organizer>,
    pub attendees: Vec<Attendee>,
    pub categories: Vec<String>,
}

impl CalendarEvent {
    pub fn new(
        id: impl Into<String>,
        summary: impl Into<String>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into(),
            summary: summary.into(),
            description: None,
            location: None,
            start_time,
            end_time,
            is_all_day: false,
            recurrence: None,
            organizer: None,
            attendees: Vec::new(),
            categories: Vec::new(),
        }
    }

    /// Returns duration in minutes.
    pub fn duration_minutes(&self) -> i64 {
        self.end_time
            .signed_duration_since(self.start_time)
            .num_minutes()
            .max(0)
    }

    /// Formats time span for display.
    pub fn format_timespan(&self) -> String {
        if self.is_all_day {
            format!("All-Day ({})", self.start_time.format("%Y-%m-%d"))
        } else {
            format!(
                "{} - {}",
                self.start_time.format("%Y-%m-%d %H:%M"),
                self.end_time.format("%H:%M")
            )
        }
    }

    /// Checks if this event conflicts (overlaps) in time with another event.
    pub fn overlaps_with(&self, other: &CalendarEvent) -> bool {
        if self.id == other.id || self.is_all_day || other.is_all_day {
            return false;
        }
        self.start_time < other.end_time && other.start_time < self.end_time
    }
}
