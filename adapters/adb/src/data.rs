use crate::client::AdbClient;
use anyhow::Result;
use chrono::{TimeZone, Utc};
use domain::{CallLog, Contact, DeviceId, Sms};
use ports::DataProviderPort;

pub struct AdbDataProvider {
    client: AdbClient,
}

impl AdbDataProvider {
    pub fn new() -> Self {
        Self {
            client: AdbClient::new(),
        }
    }

    fn extract_value(line: &str, key: &str) -> Option<String> {
        let key_with_eq = format!("{}=", key);
        if let Some(start) = line.find(&key_with_eq) {
            let value_part = &line[start + key_with_eq.len()..];
            let value = if let Some(end) = value_part.find(", ") {
                value_part[..end].trim().to_string()
            } else {
                value_part.trim().to_string()
            };

            // Jika ADB mengembalikan literal "null" atau string kosong, anggap sebagai None
            if value.to_lowercase() == "null" || value.is_empty() {
                return None;
            }
            return Some(value);
        }
        None
    }

    fn safe_content_query(&self, device_id: &DeviceId, uri: &str, projection: &str) -> Result<String> {
        let output = self.client.shell(
            &device_id.0,
            &format!("content query --uri {} --projection {}", uri, projection),
        );

        match output {
            Ok(out) => {
                if out.contains("Permission denied") || out.contains("Error") {
                    tracing::warn!("ADB query warning for {}: {}", uri, out.trim());
                    Ok(String::new())
                } else {
                    Ok(out)
                }
            },
            Err(e) => {
                tracing::error!("ADB query failed for {}: {}", uri, e);
                Ok(String::new()) // Return empty instead of error to keep backup alive
            }
        }
    }
}

impl Default for AdbDataProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DataProviderPort for AdbDataProvider {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>> {
        let output = self.safe_content_query(device_id, "content://com.android.contacts/data", "display_name:data1:mimetype")?;
        let mut contacts_map = std::collections::HashMap::new();

        for line in output.lines() {
            let name = Self::extract_value(line, "display_name").unwrap_or_else(|| "Unknown".to_string());
            let value = Self::extract_value(line, "data1");
            let mimetype = Self::extract_value(line, "mimetype").unwrap_or_default();

            if let Some(val) = value {
                let contact = contacts_map.entry(name.clone()).or_insert(Contact {
                    name,
                    phones: vec![],
                    emails: vec![],
                    addresses: vec![],
                    organizations: vec![],
                    notes: vec![],
                });

                if mimetype.contains("phone") {
                    if !contact.phones.contains(&val) { contact.phones.push(val); }
                } else if mimetype.contains("email") {
                    if !contact.emails.contains(&val) { contact.emails.push(val); }
                }
            }
        }
        Ok(contacts_map.into_values().collect())
    }

    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>> {
        let output = self.safe_content_query(device_id, "content://sms", "address:body:date:type")?;
        let mut messages = Vec::new();
        for line in output.lines() {
            if let (Some(address), Some(body), Some(date_str)) = (
                Self::extract_value(line, "address"),
                Self::extract_value(line, "body"),
                Self::extract_value(line, "date"),
            ) {
                let timestamp = date_str.parse::<i64>().unwrap_or(0);
                let type_code = Self::extract_value(line, "type").and_then(|s| s.parse().ok()).unwrap_or(1);
                messages.push(Sms {
                    address,
                    body: body.replace("\\n", "\n"),
                    date: Utc.timestamp_opt(timestamp / 1000, 0).single().unwrap_or_else(Utc::now),
                    type_code,
                });
            }
        }
        messages.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(messages)
    }

    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>> {
        let output = self.safe_content_query(device_id, "content://call_log/calls", "number:date:duration:type:name:geocoded_location")?;
        let mut logs = Vec::new();
        for line in output.lines() {
            if let (Some(number), Some(date_str), Some(duration_str)) = (
                Self::extract_value(line, "number"),
                Self::extract_value(line, "date"),
                Self::extract_value(line, "duration"),
            ) {
                let timestamp = date_str.parse::<i64>().unwrap_or(0);
                logs.push(CallLog {
                    number,
                    name: Self::extract_value(line, "name"),
                    date: Utc.timestamp_opt(timestamp / 1000, 0).single().unwrap_or_else(Utc::now),
                    duration_seconds: duration_str.parse().unwrap_or(0),
                    type_code: Self::extract_value(line, "type").and_then(|s| s.parse().ok()).unwrap_or(1),
                    location: Self::extract_value(line, "geocoded_location"),
                });
            }
        }
        Ok(logs)
    }
}
