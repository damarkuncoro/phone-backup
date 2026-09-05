use crate::domain::{CallLogItem, CallType};
use chrono::{DateTime, Utc};

/// Parser for CSV formatted call records.
pub struct CsvCallParser;

impl CsvCallParser {
    /// Parses CSV lines into `CallLogItem` list.
    /// Expected format: `id,number,name,type,duration,timestamp`
    pub fn parse(csv_content: &str) -> Vec<CallLogItem> {
        let mut items = Vec::new();
        let mut lines = csv_content.lines();

        // Skip header if present
        let first = match lines.next() {
            Some(l) => l,
            None => return items,
        };

        let is_header = first.to_lowercase().contains("number") || first.to_lowercase().contains("phone");
        if !is_header {
            if let Some(item) = Self::parse_line(first, 1) {
                items.push(item);
            }
        }

        for (idx, line) in lines.enumerate() {
            let row_idx = idx + 2;
            if let Some(item) = Self::parse_line(line, row_idx) {
                items.push(item);
            }
        }

        items
    }

    fn parse_line(line: &str, row_idx: usize) -> Option<CallLogItem> {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() < 4 {
            return None;
        }

        let phone_number = parts[0].to_string();
        if phone_number.is_empty() {
            return None;
        }

        let contact_name = if !parts[1].is_empty() {
            Some(parts[1].to_string())
        } else {
            None
        };

        let call_type = CallType::from_str_loose(parts[2]);
        let duration_secs: u64 = parts[3].parse().unwrap_or(0);
        let timestamp = if parts.len() > 4 {
            DateTime::parse_from_rfc3339(parts[4])
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now())
        } else {
            Utc::now()
        };

        let mut item = CallLogItem::new(
            format!("csv_call_{}", row_idx),
            phone_number,
            call_type,
            timestamp,
            duration_secs,
        );
        if let Some(name) = contact_name {
            item = item.with_name(name);
        }

        Some(item)
    }
}
