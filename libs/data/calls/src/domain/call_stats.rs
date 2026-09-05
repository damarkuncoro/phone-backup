use serde::{Deserialize, Serialize};

/// Summary metrics and analytical statistics for call history.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CallStats {
    pub total_calls: usize,
    pub total_duration_secs: u64,
    pub incoming_count: usize,
    pub outgoing_count: usize,
    pub missed_count: usize,
    pub rejected_count: usize,
    pub frequent_contacts: Vec<FrequentContact>,
}

/// Represents call activity aggregated by contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrequentContact {
    pub contact_name: Option<String>,
    pub phone_number: String,
    pub call_count: usize,
    pub total_duration_secs: u64,
}

impl CallStats {
    /// Formats total talk time duration into human-readable string.
    pub fn format_total_duration(&self) -> String {
        let hours = self.total_duration_secs / 3600;
        let mins = (self.total_duration_secs % 3600) / 60;
        let secs = self.total_duration_secs % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, mins, secs)
        } else if mins > 0 {
            format!("{}m {}s", mins, secs)
        } else {
            format!("{}s", secs)
        }
    }

    /// Computes percentage of missed calls.
    pub fn missed_percentage(&self) -> f32 {
        if self.total_calls == 0 {
            0.0
        } else {
            (self.missed_count as f32 / self.total_calls as f32) * 100.0
        }
    }
}
