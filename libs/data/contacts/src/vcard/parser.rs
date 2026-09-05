use super::photo::PhotoHandler;
use crate::model::fields::{EmailAddress, EmailType, Organization, PhoneNumber, PhoneType, PostalAddress};
use crate::model::Contact;
use anyhow::Result;

pub struct VCardParser;

impl VCardParser {
    /// Parses a complete multi-card vCard string into a Vec<Contact>.
    pub fn parse_str(vcard_content: &str) -> Result<Vec<Contact>> {
        let unfolded = Self::unfold_lines(vcard_content);
        let mut contacts = Vec::new();
        let mut current_contact: Option<Contact> = None;

        for raw_line in unfolded {
            let line = raw_line.trim();
            if line.eq_ignore_ascii_case("BEGIN:VCARD") {
                current_contact = Some(Contact::new(""));
                continue;
            }

            if line.eq_ignore_ascii_case("END:VCARD") {
                if let Some(mut c) = current_contact.take() {
                    if c.display_name.is_empty() {
                        let full = c.structured_name.full_name();
                        if !full.is_empty() {
                            c.display_name = full;
                        }
                    }
                    if !c.is_empty() {
                        contacts.push(c);
                    }
                }
                continue;
            }

            if let Some(ref mut c) = current_contact {
                Self::parse_line(c, line);
            }
        }

        Ok(contacts)
    }

    fn parse_line(c: &mut Contact, line: &str) {
        let (prop_and_params, val) = match line.split_once(':') {
            Some(pair) => pair,
            None => return,
        };

        let mut parts = prop_and_params.split(';');
        let prop_name = parts.next().unwrap_or("").trim().to_uppercase();
        let params: Vec<&str> = parts.collect();

        match prop_name.as_str() {
            "FN" => c.display_name = val.trim().to_string(),
            "N" => {
                let n_parts: Vec<&str> = val.split(';').collect();
                if let Some(fam) = n_parts.first() {
                    if !fam.is_empty() { c.structured_name.family_name = Some(fam.to_string()); }
                }
                if let Some(giv) = n_parts.get(1) {
                    if !giv.is_empty() { c.structured_name.given_name = Some(giv.to_string()); }
                }
                if let Some(mid) = n_parts.get(2) {
                    if !mid.is_empty() { c.structured_name.middle_name = Some(mid.to_string()); }
                }
                if let Some(pre) = n_parts.get(3) {
                    if !pre.is_empty() { c.structured_name.prefix = Some(pre.to_string()); }
                }
                if let Some(suf) = n_parts.get(4) {
                    if !suf.is_empty() { c.structured_name.suffix = Some(suf.to_string()); }
                }
            }
            "TEL" => {
                let p_type = if params.iter().any(|p| p.to_uppercase().contains("HOME")) {
                    PhoneType::Home
                } else if params.iter().any(|p| p.to_uppercase().contains("WORK")) {
                    PhoneType::Work
                } else {
                    PhoneType::Mobile
                };
                c.phone_numbers.push(PhoneNumber::new(val.trim(), p_type));
            }
            "EMAIL" => {
                let e_type = if params.iter().any(|p| p.to_uppercase().contains("WORK")) {
                    EmailType::Work
                } else {
                    EmailType::Personal
                };
                c.emails.push(EmailAddress::new(val.trim(), e_type));
            }
            "ORG" => {
                let org = c.organization.get_or_insert(Organization {
                    company: None, title: None, department: None,
                });
                org.company = Some(val.trim().to_string());
            }
            "TITLE" => {
                let org = c.organization.get_or_insert(Organization {
                    company: None, title: None, department: None,
                });
                org.title = Some(val.trim().to_string());
            }
            "ADR" => {
                let adr_parts: Vec<&str> = val.split(';').collect();
                let addr = PostalAddress {
                    street: adr_parts.get(2).filter(|s| !s.is_empty()).map(|s| s.to_string()),
                    city: adr_parts.get(3).filter(|s| !s.is_empty()).map(|s| s.to_string()),
                    region: adr_parts.get(4).filter(|s| !s.is_empty()).map(|s| s.to_string()),
                    postal_code: adr_parts.get(5).filter(|s| !s.is_empty()).map(|s| s.to_string()),
                    country: adr_parts.get(6).filter(|s| !s.is_empty()).map(|s| s.to_string()),
                    label: None,
                };
                c.addresses.push(addr);
            }
            "NOTE" => c.notes = Some(val.replace("\\n", "\n")),
            "BDAY" => c.birthday = Some(val.trim().to_string()),
            "PHOTO" => {
                if val.starts_with("data:") {
                    if let Some((meta, b64)) = val.split_once(',') {
                        let mime = meta.trim_start_matches("data:").trim_end_matches(";base64");
                        if let Some(photo) = PhotoHandler::decode_base64(b64, Some(mime)) {
                            c.photos.push(photo);
                        }
                    }
                } else if let Some(photo) = PhotoHandler::decode_base64(val, None) {
                    c.photos.push(photo);
                }
            }
            _ => {}
        }
    }

    fn unfold_lines(content: &str) -> Vec<String> {
        let mut lines = Vec::new();
        for raw in content.lines() {
            if (raw.starts_with(' ') || raw.starts_with('\t')) && !lines.is_empty() {
                let last: &mut String = lines.last_mut().unwrap();
                last.push_str(&raw[1..]);
            } else if !raw.trim().is_empty() {
                lines.push(raw.to_string());
            }
        }
        lines
    }
}
