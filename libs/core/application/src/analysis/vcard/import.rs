use anyhow::Result;
use domain::{Contact, ContactEmail, ContactName, ContactOrganization, ContactPhone, ContactUrl};

pub fn import_from_vcard(vcard_data: &str) -> Result<Vec<Contact>> {
    let mut contacts = Vec::new();

    for block in vcard_data.split("BEGIN:VCARD") {
        let trimmed = block.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut display_name = String::new();
        let mut notes: Option<String> = None;
        let mut names = Vec::new();
        let mut phones = Vec::new();
        let mut emails = Vec::new();
        let addresses = Vec::new();
        let mut organizations = Vec::new();
        let mut urls = Vec::new();

        for line in trimmed.lines() {
            let line_str = line.trim();
            if line_str.starts_with("FN:") {
                display_name = line_str[3..].to_string();
            } else if line_str.starts_with("N:") {
                let parts: Vec<&str> = line_str[2..].split(';').collect();
                let family_name = parts.get(0).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let given_name = parts.get(1).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let middle_name = parts.get(2).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let prefix = parts.get(3).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let suffix = parts.get(4).filter(|s| !s.is_empty()).map(|s| s.to_string());

                names.push(ContactName {
                    display_name: if display_name.is_empty() {
                        given_name.clone()
                    } else {
                        Some(display_name.clone())
                    },
                    given_name,
                    middle_name,
                    family_name,
                    prefix,
                    suffix,
                });
            } else if line_str.starts_with("TEL") {
                if let Some(val_idx) = line_str.find(':') {
                    let raw_value = line_str[val_idx + 1..].to_string();
                    let phone_type = if line_str.contains("CELL") || line_str.contains("cell") {
                        Some("mobile".to_string())
                    } else if line_str.contains("WORK") || line_str.contains("work") {
                        Some("work".to_string())
                    } else {
                        Some("home".to_string())
                    };

                    phones.push(ContactPhone {
                        raw_value: raw_value.clone(),
                        normalized_value: Some(raw_value.replace([' ', '-', '(', ')'], "")),
                        phone_type,
                        label: None,
                        is_primary: phones.is_empty(),
                    });
                }
            } else if line_str.starts_with("EMAIL") {
                if let Some(val_idx) = line_str.find(':') {
                    let email_val = line_str[val_idx + 1..].to_string();
                    emails.push(ContactEmail {
                        value: email_val,
                        email_type: Some("home".to_string()),
                        label: None,
                        is_primary: emails.is_empty(),
                    });
                }
            } else if line_str.starts_with("ORG:") {
                organizations.push(ContactOrganization {
                    company_name: Some(line_str[4..].to_string()),
                    department: None,
                    title: None,
                    job_description: None,
                    org_type: None,
                    label: None,
                });
            } else if line_str.starts_with("URL:") {
                urls.push(ContactUrl {
                    url: line_str[4..].to_string(),
                    url_type: None,
                    label: None,
                });
            } else if line_str.starts_with("NOTE:") {
                notes = Some(line_str[5..].replace("\\n", "\n"));
            }
        }

        if display_name.is_empty() {
            if let Some(n) = names.first() {
                display_name = format!(
                    "{} {}",
                    n.given_name.as_deref().unwrap_or(""),
                    n.family_name.as_deref().unwrap_or("")
                )
                .trim()
                .to_string();
            }
        }

        if !display_name.is_empty() || !phones.is_empty() {
            contacts.push(Contact {
                id: uuid::Uuid::new_v4().to_string(),
                snapshot_id: None,
                source_id: None,
                display_name: if display_name.is_empty() {
                    "Unnamed Contact".to_string()
                } else {
                    display_name
                },
                notes,
                source: "vcard_import".to_string(),
                source_account: None,
                content_hash: None,
                metadata_json: None,
                names,
                phones,
                emails,
                addresses,
                organizations,
                urls,
                events: Vec::new(),
                photos: Vec::new(),
                labels: Vec::new(),
            });
        }
    }

    Ok(contacts)
}
