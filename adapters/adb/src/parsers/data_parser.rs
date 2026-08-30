use domain::{CallLog, Contact, DeviceId, Sms, ContactName, ContactPhone, ContactEmail, ContactAddress, ContactOrganization, ContactUrl, ContactEvent};
use chrono::{TimeZone, Utc};
use sha2::{Sha256, Digest};

pub struct DataParser;

impl DataParser {
    pub fn extract_value(line: &str, key: &str) -> Option<String> {
        let key_with_eq = format!("{}=", key);
        if let Some(start) = line.find(&key_with_eq) {
            let value_part = &line[start + key_with_eq.len()..];
            let value = if let Some(end) = value_part.find(", ") {
                value_part[..end].trim().to_string()
            } else {
                value_part.trim().to_string()
            };

            if value.to_lowercase() == "null" || value.is_empty() {
                return None;
            }
            return Some(value);
        }
        None
    }

    pub fn parse_contacts(_device_id: &DeviceId, output: &str) -> Vec<Contact> {
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
                id: uuid::Uuid::new_v4().to_string(),
                snapshot_id: None,
                source_id: Some(contact_id),
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

        // Calculate content hashes for deduplication
        for contact in contacts_map.values_mut() {
            let json = serde_json::to_string(&contact).unwrap_or_default();
            contact.content_hash = Some(Sha256::digest(json.as_bytes()).iter().map(|b| format!("{:02x}", b)).collect());
        }

        contacts_map.into_values().collect()
    }

    pub fn parse_sms(output: &str) -> Vec<Sms> {
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
        messages
    }

    pub fn parse_filesystem_scan(device_id: &DeviceId, stdout: &str) -> Vec<domain::FileEntry> {
        stdout.lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() < 3 { return None; }

                let path = parts[0].to_string();
                let size_bytes = parts[1].parse::<u64>().unwrap_or(0);
                let mtime_unix = parts[2].parse::<i64>().unwrap_or(0);

                let modified_at = Utc.timestamp_opt(mtime_unix, 0)
                    .single()
                    .unwrap_or_else(Utc::now);

                let name = path.split('/').last().unwrap_or("").to_string();
                let mime_type = mime_guess::from_path(&path).first_or_octet_stream().to_string();

                Some(domain::FileEntry {
                    id: domain::FileId(path.clone()),
                    device_id: device_id.clone(),
                    path,
                    name,
                    size_bytes,
                    modified_at,
                    mime_type,
                    permissions: String::new(),
                    hash_sha256: None,
                    thumbnail_hash: None,
                    media_info: None,
                })
            })
            .collect()
    }

    pub fn parse_mediastore(device_id: &DeviceId, output: &str) -> Vec<domain::FileEntry> {
        output.lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let path = Self::extract_value(line, "_data")?;
                let size = Self::extract_value(line, "_size").and_then(|s| s.parse().ok()).unwrap_or(0);
                let mtime = Self::extract_value(line, "date_modified").and_then(|s| s.parse().ok()).unwrap_or(0);
                let mime = Self::extract_value(line, "mime_type").unwrap_or_default();

                let width = Self::extract_value(line, "width").and_then(|s| s.parse().ok());
                let height = Self::extract_value(line, "height").and_then(|s| s.parse().ok());
                let taken_at_ms = Self::extract_value(line, "datetaken").and_then(|s| s.parse::<i64>().ok());
                let lat = Self::extract_value(line, "latitude").and_then(|s| s.parse().ok());
                let lon = Self::extract_value(line, "longitude").and_then(|s| s.parse().ok());

                let modified_at = Utc.timestamp_opt(mtime, 0).single().unwrap_or_else(Utc::now);
                let taken_at = taken_at_ms.and_then(|ms| Utc.timestamp_opt(ms / 1000, 0).single());

                let media_info = if width.is_some() || height.is_some() || taken_at.is_some() || lat.is_some() {
                    Some(domain::MediaInfo {
                        width,
                        height,
                        taken_at,
                        latitude: lat,
                        longitude: lon,
                        ..Default::default()
                    })
                } else {
                    None
                };

                Some(domain::FileEntry {
                    id: domain::FileId(path.clone()),
                    device_id: device_id.clone(),
                    path: path.clone(),
                    name: path.split('/').last().unwrap_or("").to_string(),
                    size_bytes: size,
                    modified_at,
                    mime_type: mime,
                    permissions: String::new(),
                    hash_sha256: None,
                    thumbnail_hash: None,
                    media_info,
                })
            })
            .collect()
    }

    pub fn parse_call_logs(output: &str) -> Vec<CallLog> {
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
        logs
    }
}
