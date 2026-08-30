use anyhow::Result;
use domain::{
    Contact, ContactEmail, ContactName, ContactOrganization, ContactPhone,
    ContactUrl,
};

pub struct VCardEngine;

impl VCardEngine {
    /// Export a slice of domain `Contact` objects into standard RFC 6350 vCard 4.0 string format.
    pub fn export_to_vcard(contacts: &[Contact]) -> String {
        let mut vcard_out = String::new();

        for contact in contacts {
            vcard_out.push_str("BEGIN:VCARD\r\n");
            vcard_out.push_str("VERSION:4.0\r\n");

            // N: Family;Given;Middle;Prefix;Suffix
            if let Some(name) = contact.names.first() {
                let family = name.family_name.as_deref().unwrap_or("");
                let given = name.given_name.as_deref().unwrap_or("");
                let middle = name.middle_name.as_deref().unwrap_or("");
                let prefix = name.prefix.as_deref().unwrap_or("");
                let suffix = name.suffix.as_deref().unwrap_or("");
                vcard_out.push_str(&format!(
                    "N:{};{};{};{};{}\r\n",
                    family, given, middle, prefix, suffix
                ));
            }

            vcard_out.push_str(&format!("FN:{}\r\n", contact.display_name));

            // TEL
            for phone in &contact.phones {
                let ptype = phone
                    .phone_type
                    .as_deref()
                    .unwrap_or("cell")
                    .to_uppercase();
                vcard_out.push_str(&format!("TEL;TYPE={}:{}\r\n", ptype, phone.raw_value));
            }

            // EMAIL
            for email in &contact.emails {
                let etype = email
                    .email_type
                    .as_deref()
                    .unwrap_or("INTERNET")
                    .to_uppercase();
                vcard_out.push_str(&format!("EMAIL;TYPE={}:{}\r\n", etype, email.value));
            }

            // ADR: ;;Street;City;Region;PostalCode;Country
            for adr in &contact.addresses {
                let street = adr.street.as_deref().unwrap_or("");
                let city = adr.city.as_deref().unwrap_or("");
                let region = adr.region.as_deref().unwrap_or("");
                let postal = adr.postal_code.as_deref().unwrap_or("");
                let country = adr.country.as_deref().unwrap_or("");
                vcard_out.push_str(&format!(
                    "ADR:;;{};{};{};{};{}\r\n",
                    street, city, region, postal, country
                ));
            }

            // ORG & TITLE
            for org in &contact.organizations {
                if let Some(comp) = &org.company_name {
                    vcard_out.push_str(&format!("ORG:{}\r\n", comp));
                }
                if let Some(title) = &org.title {
                    vcard_out.push_str(&format!("TITLE:{}\r\n", title));
                }
            }

            // URL
            for url in &contact.urls {
                vcard_out.push_str(&format!("URL:{}\r\n", url.url));
            }

            // NOTE
            if let Some(notes) = &contact.notes {
                let escaped = notes.replace('\n', "\\n");
                vcard_out.push_str(&format!("NOTE:{}\r\n", escaped));
            }

            vcard_out.push_str("END:VCARD\r\n\r\n");
        }

        vcard_out
    }

    /// Parse vCard string data into domain `Contact` objects.
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
}
