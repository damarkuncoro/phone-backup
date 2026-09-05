use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Summary metrics and statistics for calendar events.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalendarStats {
    pub total_events: usize,
    pub recurring_count: usize,
    pub all_day_count: usize,
    pub upcoming_count: usize,
    pub past_count: usize,
    pub categories_count: HashMap<String, usize>,
}
