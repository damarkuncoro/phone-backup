use crate::model::{CallEntry, CallType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CallStatsSummary {
    pub total_calls: usize,
    pub incoming_count: usize,
    pub outgoing_count: usize,
    pub missed_count: usize,
    pub total_duration_seconds: u64,
    pub average_duration_seconds: u64,
    pub top_numbers: Vec<(String, usize)>,
}

pub struct CallLogAnalytics;

impl CallLogAnalytics {
    pub fn compute_summary(calls: &[CallEntry]) -> CallStatsSummary {
        let total_calls = calls.len();
        if total_calls == 0 {
            return CallStatsSummary::default();
        }

        let mut incoming_count = 0;
        let mut outgoing_count = 0;
        let mut missed_count = 0;
        let mut total_duration = 0;
        let mut number_frequency: HashMap<String, usize> = HashMap::new();

        for call in calls {
            *number_frequency.entry(call.number.clone()).or_insert(0) += 1;
            total_duration += call.duration_seconds;

            match call.call_type {
                CallType::Incoming => incoming_count += 1,
                CallType::Outgoing => outgoing_count += 1,
                CallType::Missed | CallType::Rejected => missed_count += 1,
                _ => {}
            }
        }

        let mut top_numbers: Vec<(String, usize)> = number_frequency.into_iter().collect();
        top_numbers.sort_by_key(|b| std::cmp::Reverse(b.1));
        top_numbers.truncate(5);

        let connected_count = incoming_count + outgoing_count;
        let average_duration_seconds = if connected_count > 0 {
            total_duration / (connected_count as u64)
        } else {
            0
        };

        CallStatsSummary {
            total_calls,
            incoming_count,
            outgoing_count,
            missed_count,
            total_duration_seconds: total_duration,
            average_duration_seconds,
            top_numbers,
        }
    }
}
