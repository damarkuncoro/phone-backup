use rusqlite::params;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use domain::{SnapshotId, Contact};
use ports::ContactRepositoryPort;
use chrono::Utc;

pub struct ContactRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl ContactRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }

    fn map_row_to_contact(row: &rusqlite::Row) -> rusqlite::Result<Contact> {
        let id: String = row.get(0)?;
        let snapshot_id: String = row.get(1)?;
        let source_id: Option<String> = row.get(2)?;
        let display_name: String = row.get(3)?;
        let notes: Option<String> = row.get(4)?;
        let source: String = row.get(5)?;
        let source_account: Option<String> = row.get(6)?;
        let content_hash: Option<String> = row.get(7)?;
        let metadata_json: Option<String> = row.get(8)?;

        let names_json: String = row.get("names_json")?;
        let phones_json: String = row.get("phones_json")?;
        let emails_json: String = row.get("emails_json")?;
        let addresses_json: String = row.get("addresses_json")?;
        let organizations_json: String = row.get("organizations_json")?;
        let urls_json: String = row.get("urls_json")?;
        let events_json: String = row.get("events_json")?;
        let photos_json: String = row.get("photos_json")?;
        let labels_json: String = row.get("labels_json")?;

        Ok(Contact {
            id,
            snapshot_id: Some(snapshot_id),
            source_id,
            display_name,
            notes,
            source,
            source_account,
            content_hash,
            metadata_json,
            names: serde_json::from_str(&names_json).unwrap_or_default(),
            phones: serde_json::from_str(&phones_json).unwrap_or_default(),
            emails: serde_json::from_str(&emails_json).unwrap_or_default(),
            addresses: serde_json::from_str(&addresses_json).unwrap_or_default(),
            organizations: serde_json::from_str(&organizations_json).unwrap_or_default(),
            urls: serde_json::from_str(&urls_json).unwrap_or_default(),
            events: serde_json::from_str(&events_json).unwrap_or_default(),
            photos: serde_json::from_str(&photos_json).unwrap_or_default(),
            labels: serde_json::from_str(&labels_json).unwrap_or_default(),
        })
    }

    const FULL_CONTACT_SELECT: &'static str = r#"
        SELECT
            c.*,
            (SELECT COALESCE(json_group_array(json_object(
                'display_name', display_name, 'given_name', given_name, 'middle_name', middle_name,
                'family_name', family_name, 'prefix', prefix, 'suffix', suffix
            )), '[]') FROM contact_names WHERE contact_id = c.id) as names_json,
            (SELECT COALESCE(json_group_array(json_object(
                'raw_value', raw_value, 'normalized_value', normalized_value,
                'phone_type', type, 'label', label,
                'is_primary', CASE WHEN is_primary != 0 THEN json('true') ELSE json('false') END
            )), '[]') FROM contact_phones WHERE contact_id = c.id) as phones_json,
            (SELECT COALESCE(json_group_array(json_object(
                'value', value, 'email_type', type, 'label', label,
                'is_primary', CASE WHEN is_primary != 0 THEN json('true') ELSE json('false') END
            )), '[]') FROM contact_emails WHERE contact_id = c.id) as emails_json,
            (SELECT COALESCE(json_group_array(json_object(
                'formatted_address', formatted_address, 'street', street, 'city', city,
                'region', region, 'postal_code', postal_code, 'country', country,
                'country_code', country_code, 'address_type', type, 'label', label
            )), '[]') FROM contact_addresses WHERE contact_id = c.id) as addresses_json,
            (SELECT COALESCE(json_group_array(json_object(
                'company_name', company_name, 'department', department, 'title', title,
                'job_description', job_description, 'org_type', type, 'label', label
            )), '[]') FROM contact_organizations WHERE contact_id = c.id) as organizations_json,
            (SELECT COALESCE(json_group_array(json_object(
                'url', url, 'url_type', type, 'label', label
            )), '[]') FROM contact_urls WHERE contact_id = c.id) as urls_json,
            (SELECT COALESCE(json_group_array(json_object(
                'event_type', event_type, 'event_date', event_date, 'label', label
            )), '[]') FROM contact_events WHERE contact_id = c.id) as events_json,
            (SELECT COALESCE(json_group_array(json_object(
                'file_id', file_id, 'photo_hash', photo_hash, 'mime_type', mime_type,
                'is_primary', CASE WHEN is_primary != 0 THEN json('true') ELSE json('false') END
            )), '[]') FROM contact_photos WHERE contact_id = c.id) as photos_json,
            (SELECT COALESCE(json_group_array(cl.name), '[]')
             FROM contact_labels cl JOIN contact_label_members clm ON cl.id = clm.label_id
             WHERE clm.contact_id = c.id) as labels_json
        FROM contacts c
    "#;
}

impl ContactRepositoryPort for ContactRepository {
    fn save_contact(&self, snapshot_id: &SnapshotId, contact: &Contact) -> anyhow::Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        let db_id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();

        tx.execute(
            "INSERT INTO contacts (id, snapshot_id, source_id, display_name, notes, source, source_account, content_hash, metadata_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                db_id, snapshot_id.0, contact.id, contact.display_name,
                contact.notes, contact.source, contact.source_account,
                contact.content_hash, contact.metadata_json, created_at
            ],
        )?;

        for name in &contact.names {
            tx.execute(
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
            tx.execute(
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
            tx.execute(
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
            tx.execute(
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
            tx.execute(
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
            tx.execute(
                "INSERT INTO contact_urls (id, contact_id, url, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![uuid::Uuid::new_v4().to_string(), db_id, url.url, url.url_type, url.label],
            )?;
        }

        for event in &contact.events {
            tx.execute(
                "INSERT INTO contact_events (id, contact_id, event_type, event_date, label)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![uuid::Uuid::new_v4().to_string(), db_id, event.event_type, event.event_date, event.label],
            )?;
        }

        for photo in &contact.photos {
            tx.execute(
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
            tx.execute(
                "INSERT INTO contact_labels (id, snapshot_id, name, source, source_account)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![label_id, snapshot_id.0, label_name, contact.source, contact.source_account],
            )?;

            tx.execute(
                "INSERT INTO contact_label_members (contact_id, label_id) VALUES (?1, ?2)",
                params![db_id, label_id],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    fn get_snapshot_contacts(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<Contact>> {
        let conn = self.pool.get()?;
        let query = format!("{} WHERE c.snapshot_id = ?1", Self::FULL_CONTACT_SELECT);
        let mut stmt = conn.prepare(&query)?;

        let contacts = stmt.query_map([&snapshot_id.0], Self::map_row_to_contact)?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(contacts)
    }

    fn search_contacts(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, Contact)>> {
        let conn = self.pool.get()?;

        // FTS Search
        let fts_query = format!("\"{}\"*", query.replace("\"", "\"\""));
        let sql = format!(
            "{} JOIN contacts_fts fts ON c.rowid = fts.rowid WHERE contacts_fts MATCH ?1 ORDER BY rank",
            Self::FULL_CONTACT_SELECT
        );

        let mut stmt = conn.prepare(&sql)?;
        let mut results: Vec<(SnapshotId, Contact)> = stmt.query_map([fts_query], |row| {
            let snapshot_id: String = row.get(1)?;
            Ok((SnapshotId(snapshot_id), Self::map_row_to_contact(row)?))
        })?.filter_map(|r| r.ok()).collect();

        // Fallback to LIKE
        if results.is_empty() {
            let pattern = format!("%{}%", query);
            let sql_like = format!(
                "{} WHERE c.display_name LIKE ?1 OR c.notes LIKE ?1 LIMIT 100",
                Self::FULL_CONTACT_SELECT
            );
            let mut stmt_like = conn.prepare(&sql_like)?;
            results = stmt_like.query_map([pattern], |row| {
                let snapshot_id: String = row.get(1)?;
                Ok((SnapshotId(snapshot_id), Self::map_row_to_contact(row)?))
            })?.filter_map(|r| r.ok()).collect();
        }

        Ok(results)
    }
}
