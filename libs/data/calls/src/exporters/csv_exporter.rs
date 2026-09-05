use crate::domain::CallLogItem;

/// Exporter for generating standard CSV exports of call records.
pub struct CsvCallExporter;

impl CsvCallExporter {
    /// Serializes a list of call log items to CSV string.
    pub fn export(items: &[CallLogItem]) -> String {
        let mut out = String::from("Number,Contact Name,Type,Duration (s),Timestamp,SIM Slot\n");
        for item in items {
            let name = item.contact_name.as_deref().unwrap_or("");
            let sim = item
                .sim_slot
                .map(|s| s.to_string())
                .unwrap_or_else(|| "-".to_string());

            out.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",{},\"{}\",\"{}\"\n",
                item.phone_number,
                name.replace('"', "\"\""),
                item.call_type.display_name(),
                item.duration_secs,
                item.timestamp.to_rfc3339(),
                sim
            ));
        }
        out
    }
}
