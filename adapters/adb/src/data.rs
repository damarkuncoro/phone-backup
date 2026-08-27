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
            if let Some(end) = value_part.find(", ") {
                return Some(value_part[..end].trim().to_string());
            } else {
                return Some(value_part.trim().to_string());
            }
        }
        None
    }
}

impl Default for AdbDataProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DataProviderPort for AdbDataProvider {
    fn list_contacts(&self, device_id: &DeviceId) -> Result<Vec<Contact>> {
        let output = self.client.shell(
            &device_id.0,
            "content query --uri content://com.android.contacts/data --projection display_name:data1",
        )?;

        let mut contacts = std::collections::HashMap::new();
        for line in output.lines() {
            if let (Some(name), Some(phone)) = (
                Self::extract_value(line, "display_name"),
                Self::extract_value(line, "data1"),
            ) {
                let contact = contacts.entry(name.clone()).or_insert(Contact {
                    name,
                    phones: vec![],
                    emails: vec![],
                });
                if !contact.phones.contains(&phone) {
                    contact.phones.push(phone);
                }
            }
        }
        Ok(contacts.into_values().collect())
    }

    fn list_sms(&self, device_id: &DeviceId) -> Result<Vec<Sms>> {
        let output = self.client.shell(
            &device_id.0,
            "content query --uri content://sms --projection address:body:date:type",
        )?;
        let mut messages = Vec::new();
        for line in output.lines() {
            if let (Some(address), Some(body), Some(date_str)) = (
                Self::extract_value(line, "address"),
                Self::extract_value(line, "body"),
                Self::extract_value(line, "date"),
            ) {
                let timestamp = date_str.parse::<i64>().unwrap_or(0);
                messages.push(Sms {
                    address,
                    body,
                    date: Utc.timestamp_opt(timestamp / 1000, 0).single().unwrap_or_else(Utc::now),
                    type_code: Self::extract_value(line, "type").and_then(|s| s.parse().ok()).unwrap_or(1),
                });
            }
        }
        Ok(messages)
    }

    fn list_call_logs(&self, device_id: &DeviceId) -> Result<Vec<CallLog>> {
        let output = self.client.shell(
            &device_id.0,
            "content query --uri content://call_log/calls --projection number:date:duration:type",
        )?;
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
                    date: Utc.timestamp_opt(timestamp / 1000, 0).single().unwrap_or_else(Utc::now),
                    duration_seconds: duration_str.parse().unwrap_or(0),
                    type_code: Self::extract_value(line, "type").and_then(|s| s.parse().ok()).unwrap_or(1),
                });
            }
        }
        Ok(logs)
    }
}
