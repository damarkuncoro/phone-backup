use crate::domain::{CalendarEvent, CalendarStats};
use chrono::Utc;
use std::collections::HashMap;

/// Domain service for analyzing calendar schedules, finding conflicts and computing stats.
pub struct CalendarAnalytics;

impl CalendarAnalytics {
    /// Computes summary metrics for calendar events.
    pub fn compute_stats(events: &[CalendarEvent]) -> CalendarStats {
        let now = Utc::now();
        let mut stats = CalendarStats {
            total_events: events.len(),
            ..Default::default()
        };

        let mut cat_map: HashMap<String, usize> = HashMap::new();

        for event in events {
            if event.recurrence.is_some() {
                stats.recurring_count += 1;
            }
            if event.is_all_day {
                stats.all_day_count += 1;
            }
            if event.end_time >= now {
                stats.upcoming_count += 1;
            } else {
                stats.past_count += 1;
            }

            for cat in &event.categories {
                *cat_map.entry(cat.clone()).or_insert(0) += 1;
            }
        }

        stats.categories_count = cat_map;
        stats
    }

    /// Finds overlapping event conflicts in the given schedule.
    pub fn find_conflicts(events: &[CalendarEvent]) -> Vec<(&CalendarEvent, &CalendarEvent)> {
        let mut conflicts = Vec::new();
        for (i, a) in events.iter().enumerate() {
            for b in events.iter().skip(i + 1) {
                if a.overlaps_with(b) {
                    conflicts.push((a, b));
                }
            }
        }
        conflicts
    }

    /// Filters calendar events by upcoming flag, category, or search query.
    pub fn filter_events(
        events: Vec<CalendarEvent>,
        upcoming_only: bool,
        category: Option<&str>,
        query: Option<&str>,
    ) -> Vec<CalendarEvent> {
        let now = Utc::now();
        let q_lower = query.map(|q| q.to_lowercase());
        let cat_lower = category.map(|c| c.to_lowercase());

        events
            .into_iter()
            .filter(|e| {
                if upcoming_only && e.end_time < now {
                    return false;
                }
                if let Some(ref cat) = cat_lower {
                    let has_cat = e.categories.iter().any(|c| c.to_lowercase() == *cat);
                    if !has_cat {
                        return false;
                    }
                }
                if let Some(ref q) = q_lower {
                    let summary_matches = e.summary.to_lowercase().contains(q);
                    let desc_matches = e
                        .description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(q))
                        .unwrap_or(false);
                    let loc_matches = e
                        .location
                        .as_ref()
                        .map(|l| l.to_lowercase().contains(q))
                        .unwrap_or(false);
                    if !summary_matches && !desc_matches && !loc_matches {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
