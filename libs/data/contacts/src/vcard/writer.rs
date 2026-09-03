use super::photo::PhotoHandler;
use super::version::VCardVersion;
use crate::model::fields::{EmailType, PhoneType};
use crate::model::Contact;

pub struct VCardWriter;

impl VCardWriter {
    /// Serializes a slice of contacts into standard vCard format string.
    pub fn write_contacts(contacts: &[Contact], version: VCardVersion) -> String {
        let mut out = String::new();
        for c in contacts {
            out.push_str(&Self::write_single(c, version));
            out.push('\n');
        }
        out
    }

    /// Serializes a single contact into a single vCard record.
    pub fn write_single(c: &Contact, version: VCardVersion) -> String {
        let mut lines = Vec::new();
        lines.push("BEGIN:VCARD".to_string());
        lines.push(format!("VERSION:{}", version.as_str()));

        // Formatted Name
        if !c.display_name.is_empty() {
            lines.push(format!("FN:{}", c.display_name));
        }

        // Structured Name N:Family;Given;Middle;Prefix;Suffix
        let sn = &c.structured_name;
        let n_val = format!(
            "{};{};{};{};{}",
            sn.family_name.as_deref().unwrap_or(""),
            sn.given_name.as_deref().unwrap_or(""),
            sn.middle_name.as_deref().unwrap_or(""),
            sn.prefix.as_deref().unwrap_or(""),
            sn.suffix.as_deref().unwrap_or("")
        );
        if n_val != ";;;;" {
            lines.push(format!("N:{}", n_val));
        }

        // Phone Numbers
        for p in &c.phone_numbers {
            let type_str = match &p.phone_type {
                PhoneType::Mobile => "CELL",
                PhoneType::Home => "HOME",
                PhoneType::Work => "WORK",
                PhoneType::Main => "MAIN",
                PhoneType::Fax => "FAX",
                PhoneType::Other(s) => s.as_str(),
            };
            lines.push(format!("TEL;TYPE={}:{}", type_str, p.raw));
        }

        // Emails
        for e in &c.emails {
            let type_str = match &e.email_type {
                EmailType::Personal => "HOME",
                EmailType::Work => "WORK",
                EmailType::Other(s) => s.as_str(),
            };
            lines.push(format!("EMAIL;TYPE={}:{}", type_str, e.email));
        }

        // Organization
        if let Some(org) = &c.organization {
            if let Some(company) = &org.company {
                lines.push(format!("ORG:{}", company));
            }
            if let Some(title) = &org.title {
                lines.push(format!("TITLE:{}", title));
            }
        }

        // Addresses
        for a in &c.addresses {
            lines.push(format!(
                "ADR:;;{};{};{};{};{}",
                a.street.as_deref().unwrap_or(""),
                a.city.as_deref().unwrap_or(""),
                a.region.as_deref().unwrap_or(""),
                a.postal_code.as_deref().unwrap_or(""),
                a.country.as_deref().unwrap_or("")
            ));
        }

        // Note & Birthday
        if let Some(note) = &c.notes {
            lines.push(format!("NOTE:{}", note.replace('\n', "\\n")));
        }
        if let Some(bday) = &c.birthday {
            lines.push(format!("BDAY:{}", bday));
        }

        // Photos
        for p in &c.photos {
            let b64 = PhotoHandler::encode_base64(p);
            match version {
                VCardVersion::V2_1 | VCardVersion::V3_0 => {
                    lines.push(format!("PHOTO;ENCODING=b;TYPE={}:{}", p.mime_type, b64));
                }
                VCardVersion::V4_0 => {
                    lines.push(format!("PHOTO:data:{};base64,{}", p.mime_type, b64));
                }
            }
        }

        lines.push("END:VCARD".to_string());
        lines.join("\r\n")
    }
}
