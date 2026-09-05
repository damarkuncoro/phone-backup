use crate::domain::{CallLogItem, CallStats, CallType, FrequentContact};
use std::collections::HashMap;

/// Domain service for computing statistical and analytical metrics over call histories.
pub struct CallAnalytics;

impl CallAnalytics {
    /// Computes full summary statistics from a list of call logs.
    pub fn compute_stats(items: &[CallLogItem]) -> CallStats {
        let mut stats = CallStats {
            total_calls: items.len(),
            ..Default::default()
        };

        let mut caller_map: HashMap<String, (Option<String>, usize, u64)> = HashMap::new();

        for item in items {
            stats.total_duration_secs += item.duration_secs;
            match item.call_type {
                CallType::Incoming => stats.incoming_count += 1,
                CallType::Outgoing => stats.outgoing_count += 1,
                CallType::Missed => stats.missed_count += 1,
                CallType::Rejected => stats.rejected_count += 1,
                _ => {}
            }

            let entry = caller_map
                .entry(item.phone_number.clone())
                .or_insert_with(|| (item.contact_name.clone(), 0, 0));
            entry.1 += 1;
            entry.2 += item.duration_secs;
            if entry.0.is_none() && item.contact_name.is_some() {
                entry.0 = item.contact_name.clone();
            }
        }

        let mut frequent: Vec<FrequentContact> = caller_map
            .into_iter()
            .map(|(number, (name, count, dur))| FrequentContact {
                contact_name: name,
                phone_number: number,
                call_count: count,
                total_duration_secs: dur,
            })
            .collect();

        frequent.sort_by(|a, b| b.call_count.cmp(&a.call_count).then_with(|| b.total_duration_secs.cmp(&a.total_duration_secs)));
        stats.frequent_contacts = frequent;

        stats
    }

    /// Filters call logs by call type, contact/number query, or minimum duration.
    pub fn filter_calls(
        items: Vec<CallLogItem>,
        target_type: Option<CallType>,
        query: Option<&str>,
        min_duration_secs: Option<u64>,
    ) -> Vec<CallLogItem> {
        let q_lower = query.map(|q| q.to_lowercase());
        items
            .into_iter()
            .filter(|item| {
                if let Some(t) = target_type {
                    if item.call_type != t {
                        return false;
                    }
                }
                if let Some(min) = min_duration_secs {
                    if item.duration_secs < min {
                        return false;
                    }
                }
                if let Some(ref q) = q_lower {
                    let num_matches = item.phone_number.to_lowercase().contains(q);
                    let name_matches = item
                        .contact_name
                        .as_ref()
                        .map(|n| n.to_lowercase().contains(q))
                        .unwrap_or(false);
                    if !num_matches && !name_matches {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
