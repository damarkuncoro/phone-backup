use crate::client::AdbClient;
use anyhow::Result;
use chrono::{TimeZone, Utc};
use domain::{CallLog, Contact, DeviceId, Sms, ContactName, ContactPhone, ContactEmail, ContactAddress, ContactOrganization, ContactUrl, ContactEvent};
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
        // Broad projection to capture most common data fields
        let projection = "contact_id:display_name:mimetype:account_name:data1:data2:data3:data4:data5:data6:data7:data8:data9:data10";
        let output = self.safe_content_query(device_id, "content://com.android.contacts/data", projection)?;

        let mut contacts_map = std::collections::HashMap::new();

        for line in output.lines() {
            let contact_id = Self::extract_value(line, "contact_id").unwrap_or_else(|| "0".to_string());
            let display_name = Self::extract_value(line, "display_name").unwrap_or_else(|| "Unknown".to_string());
            let mimetype = Self::extract_value(line, "mimetype").unwrap_or_default();
            let account_name = Self::extract_value(line, "account_name");

            let data1 = Self::extract_value(line, "data1");
            let data2 = Self::extract_value(line, "data2");
            let data3 = Self::extract_value(line, "data3");
            let data4 = Self::extract_value(line, "data4");
            let data5 = Self::extract_value(line, "data5");
            let data7 = Self::extract_value(line, "data7");
            let data8 = Self::extract_value(line, "data8");
            let data9 = Self::extract_value(line, "data9");

            let contact = contacts_map.entry(contact_id.clone()).or_insert(Contact {
                id: contact_id,
                snapshot_id: None,
                source_id: None, // Will be filled with contact_id if needed
                display_name,
                notes: None,
                source: "android".to_string(),
                source_account: account_name,
                content_hash: None,
                metadata_json: None,
                names: vec![],
                phones: vec![],
                emails: vec![],
                addresses: vec![],
                organizations: vec![],
                urls: vec![],
                events: vec![],
                photos: vec![],
                labels: vec![],
            });

            if mimetype.contains("name") {
                contact.names.push(ContactName {
                    display_name: data1,
                    given_name: data2,
                    family_name: data3,
                    prefix: data4,
                    middle_name: data5,
                    suffix: None,
                });
            } else if mimetype.contains("phone") {
                contact.phones.push(ContactPhone {
                    raw_value: data1.unwrap_or_default(),
                    normalized_value: data4,
                    phone_type: data2,
                    label: data3,
                    is_primary: false,
                });
            } else if mimetype.contains("email") {
                contact.emails.push(ContactEmail {
                    value: data1.unwrap_or_default(),
                    email_type: data2,
                    label: data3,
                    is_primary: false,
                });
            } else if mimetype.contains("postal-address") {
                contact.addresses.push(ContactAddress {
                    formatted_address: data1,
                    address_type: data2,
                    label: data3,
                    street: data4,
                    postal_code: data9,
                    city: data7,
                    region: data8,
                    country: None,
                    country_code: None,
                });
            } else if mimetype.contains("organization") {
                contact.organizations.push(ContactOrganization {
                    company_name: data1,
                    org_type: data2,
                    label: data3,
                    title: data4,
                    department: data5,
                    job_description: None,
                });
            } else if mimetype.contains("note") {
                contact.notes = data1;
            } else if mimetype.contains("website") {
                contact.urls.push(ContactUrl {
                    url: data1.unwrap_or_default(),
                    url_type: data2,
                    label: data3,
                });
            } else if mimetype.contains("event") {
                contact.events.push(ContactEvent {
                    event_date: data1.unwrap_or_default(),
                    event_type: data2.unwrap_or_else(|| "custom".to_string()),
                    label: data3,
                });
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
