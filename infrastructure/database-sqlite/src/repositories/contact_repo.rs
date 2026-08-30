use rusqlite::{params, Connection};
use domain::{SnapshotId, Contact};
use crate::mappers::AndroidMapper;
use chrono::Utc;

pub struct ContactRepository;

impl ContactRepository {
    pub fn save(conn: &Connection, snapshot_id: &SnapshotId, contact: &Contact) -> anyhow::Result<()> {
        let db_id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO contacts (id, snapshot_id, source_id, display_name, notes, source, source_account, content_hash, metadata_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                db_id, snapshot_id.0, contact.id, contact.display_name,
                contact.notes, contact.source, contact.source_account,
                contact.content_hash, contact.metadata_json, created_at
            ],
        )?;

        if let Some(name) = contact.names.first() {
            conn.execute(
                "INSERT INTO contact_names (id, contact_id, display_name, given_name, middle_name, family_name, prefix, suffix)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    uuid::Uuid::new_v4().to_string(), db_id,
                    name.display_name, name.given_name, name.middle_name,
                    name.family_name, name.prefix, name.suffix
                ],
            )?;
        }

        for phone in &contact.phones {
            conn.execute(
                "INSERT INTO contact_phones (id, contact_id, raw_value, normalized_value, type, label, is_primary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    uuid::Uuid::new_v4().to_string(), db_id,
                    phone.raw_value, phone.normalized_value, phone.phone_type,
                    phone.label, if phone.is_primary { 1 } else { 0 }
                ],
            )?;
        }

        for email in &contact.emails {
            conn.execute(
                "INSERT INTO contact_emails (id, contact_id, value, type, label, is_primary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    uuid::Uuid::new_v4().to_string(), db_id,
                    email.value, email.email_type, email.label,
                    if email.is_primary { 1 } else { 0 }
                ],
            )?;
        }

        for addr in &contact.addresses {
            conn.execute(
                "INSERT INTO contact_addresses (id, contact_id, formatted_address, street, city, region, postal_code, country, country_code, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    uuid::Uuid::new_v4().to_string(), db_id,
                    addr.formatted_address, addr.street, addr.city, addr.region,
                    addr.postal_code, addr.country, addr.country_code,
                    addr.address_type, addr.label
                ],
            )?;
        }

        for org in &contact.organizations {
            conn.execute(
                "INSERT INTO contact_organizations (id, contact_id, company_name, department, title, job_description, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    uuid::Uuid::new_v4().to_string(), db_id,
                    org.company_name, org.department, org.title,
                    org.job_description, org.org_type, org.label
                ],
            )?;
        }

        for url in &contact.urls {
            conn.execute(
                "INSERT INTO contact_urls (id, contact_id, url, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![uuid::Uuid::new_v4().to_string(), db_id, url.url, url.url_type, url.label],
            )?;
        }

        for event in &contact.events {
            conn.execute(
                "INSERT INTO contact_events (id, contact_id, event_type, event_date, label)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![uuid::Uuid::new_v4().to_string(), db_id, event.event_type, event.event_date, event.label],
            )?;
        }

        for photo in &contact.photos {
            conn.execute(
                "INSERT INTO contact_photos (id, contact_id, file_id, photo_hash, mime_type, is_primary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    uuid::Uuid::new_v4().to_string(), db_id,
                    photo.file_id, photo.photo_hash, photo.mime_type,
                    if photo.is_primary { 1 } else { 0 }
                ],
            )?;
        }

        for label_name in &contact.labels {
            let label_id = uuid::Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO contact_labels (id, snapshot_id, name, source, source_account)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![label_id, snapshot_id.0, label_name, contact.source, contact.source_account],
            )?;

            conn.execute(
                "INSERT INTO contact_label_members (contact_id, label_id) VALUES (?1, ?2)",
                params![db_id, label_id],
            )?;
        }

        Ok(())
    }

    pub fn list_by_snapshot(conn: &Connection, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<Contact>> {
        let mut stmt = conn.prepare("SELECT id FROM contacts WHERE snapshot_id = ?1")?;
        let contact_ids: Vec<String> = stmt.query_map([&snapshot_id.0], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let mut contacts = Vec::new();
        for id in contact_ids {
            if let Some(c) = Self::get_full_details(conn, &id, Some(&snapshot_id.0))? {
                contacts.push(c);
            }
        }
        Ok(contacts)
    }

    pub fn search(conn: &Connection, query: &str) -> anyhow::Result<Vec<(SnapshotId, Contact)>> {
        let mut stmt = conn.prepare(
            "SELECT id, snapshot_id FROM contacts WHERE display_name LIKE ?1"
        )?;
        let pattern = format!("%{}%", query);

        let rows: Vec<(String, String)> = stmt.query_map([pattern], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?.collect::<rusqlite::Result<Vec<_>>>()?;

        let mut results = Vec::new();
        for (id, s_id) in rows {
            if let Some(c) = Self::get_full_details(conn, &id, Some(&s_id))? {
                results.push((SnapshotId(s_id), c));
            }
        }
        Ok(results)
    }

    fn get_full_details(conn: &Connection, db_id: &str, snapshot_id: Option<&str>) -> anyhow::Result<Option<Contact>> {
        let mut stmt = conn.prepare("SELECT * FROM contacts WHERE id = ?1")?;
        let mut contact_row = stmt.query_map([db_id], |row| {
            Ok((
                row.get::<_, String>(0)?, // id
                row.get::<_, Option<String>>(2)?, // source_id
                row.get::<_, String>(3)?, // display_name
                row.get::<_, Option<String>>(4)?, // notes
                row.get::<_, String>(5)?, // source
                row.get::<_, Option<String>>(6)?, // source_account
                row.get::<_, Option<String>>(7)?, // content_hash
                row.get::<_, Option<String>>(8)?, // metadata_json
            ))
        })?;

        if let Some(Ok((id, source_id, display_name, notes, source, source_account, content_hash, metadata_json))) = contact_row.next() {
            let names = conn.prepare("SELECT * FROM contact_names WHERE contact_id = ?1")?.query_map([db_id], AndroidMapper::to_contact_name)?.collect::<rusqlite::Result<Vec<_>>>()?;
            let phones = conn.prepare("SELECT * FROM contact_phones WHERE contact_id = ?1")?.query_map([db_id], AndroidMapper::to_contact_phone)?.collect::<rusqlite::Result<Vec<_>>>()?;
            let emails = conn.prepare("SELECT * FROM contact_emails WHERE contact_id = ?1")?.query_map([db_id], AndroidMapper::to_contact_email)?.collect::<rusqlite::Result<Vec<_>>>()?;
            let addresses = conn.prepare("SELECT * FROM contact_addresses WHERE contact_id = ?1")?.query_map([db_id], AndroidMapper::to_contact_address)?.collect::<rusqlite::Result<Vec<_>>>()?;
            let organizations = conn.prepare("SELECT * FROM contact_organizations WHERE contact_id = ?1")?.query_map([db_id], AndroidMapper::to_contact_organization)?.collect::<rusqlite::Result<Vec<_>>>()?;
            let urls = conn.prepare("SELECT * FROM contact_urls WHERE contact_id = ?1")?.query_map([db_id], AndroidMapper::to_contact_url)?.collect::<rusqlite::Result<Vec<_>>>()?;
            let events = conn.prepare("SELECT * FROM contact_events WHERE contact_id = ?1")?.query_map([db_id], AndroidMapper::to_contact_event)?.collect::<rusqlite::Result<Vec<_>>>()?;
            let photos = conn.prepare("SELECT * FROM contact_photos WHERE contact_id = ?1")?.query_map([db_id], AndroidMapper::to_contact_photo)?.collect::<rusqlite::Result<Vec<_>>>()?;

            let labels = conn.prepare(
                "SELECT cl.name FROM contact_labels cl
                 JOIN contact_label_members clm ON cl.id = clm.label_id
                 WHERE clm.contact_id = ?1"
            )?.query_map([db_id], |row| row.get(0))?.collect::<rusqlite::Result<Vec<_>>>()?;

            Ok(Some(Contact {
                id,
                snapshot_id: snapshot_id.map(|s| s.to_string()),
                source_id,
                display_name,
                notes,
                source,
                source_account,
                content_hash,
                metadata_json,
                names,
                phones,
                emails,
                addresses,
                organizations,
                urls,
                events,
                photos,
                labels,
            }))
        } else {
            Ok(None)
        }
    }
}
