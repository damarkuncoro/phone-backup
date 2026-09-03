use domain::Contact;
use rusqlite::{params, Transaction};

pub struct ContactSaveHelper;

impl ContactSaveHelper {
    pub fn insert_contact_details(tx: &Transaction, new_id: &str, contact: &Contact) -> anyhow::Result<()> {
        if let Some(name) = contact.names.first() {
            tx.execute(
                "INSERT INTO contact_names (id, contact_id, display_name, given_name, middle_name, family_name, prefix, suffix)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    uuid::Uuid::new_v4().to_string(),
                    new_id,
                    name.display_name,
                    name.given_name,
                    name.middle_name,
                    name.family_name,
                    name.prefix,
                    name.suffix
                ],
            )?;
        }

        for phone in &contact.phones {
            tx.execute(
                "INSERT INTO contact_phones (id, contact_id, raw_value, normalized_value, type, label, is_primary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![uuid::Uuid::new_v4().to_string(), new_id, phone.raw_value, phone.normalized_value, phone.phone_type, phone.label, if phone.is_primary { 1 } else { 0 }],
            )?;
        }

        for email in &contact.emails {
            tx.execute(
                "INSERT INTO contact_emails (id, contact_id, value, type, label, is_primary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    uuid::Uuid::new_v4().to_string(),
                    new_id,
                    email.value,
                    email.email_type,
                    email.label,
                    if email.is_primary { 1 } else { 0 }
                ],
            )?;
        }

        for addr in &contact.addresses {
            tx.execute(
                "INSERT INTO contact_addresses (id, contact_id, formatted_address, street, city, region, postal_code, country, country_code, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![uuid::Uuid::new_v4().to_string(), new_id, addr.formatted_address, addr.street, addr.city, addr.region, addr.postal_code, addr.country, addr.country_code, addr.address_type, addr.label],
            )?;
        }

        for org in &contact.organizations {
            tx.execute(
                "INSERT INTO contact_organizations (id, contact_id, company_name, department, title, job_description, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![uuid::Uuid::new_v4().to_string(), new_id, org.company_name, org.department, org.title, org.job_description, org.org_type, org.label],
            )?;
        }

        for url in &contact.urls {
            tx.execute("INSERT INTO contact_urls (id, contact_id, url, type, label) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![uuid::Uuid::new_v4().to_string(), new_id, url.url, url.url_type, url.label],
            )?;
        }

        for event in &contact.events {
            tx.execute("INSERT INTO contact_events (id, contact_id, event_type, event_date, label) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![uuid::Uuid::new_v4().to_string(), new_id, event.event_type, event.event_date, event.label],
            )?;
        }

        for photo in &contact.photos {
            tx.execute("INSERT INTO contact_photos (id, contact_id, file_id, photo_hash, mime_type, is_primary) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![uuid::Uuid::new_v4().to_string(), new_id, photo.file_id, photo.photo_hash, photo.mime_type, if photo.is_primary { 1 } else { 0 }],
            )?;
        }

        Ok(())
    }
}
