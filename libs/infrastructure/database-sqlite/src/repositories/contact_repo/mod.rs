pub mod queries;
pub mod save;

use chrono::Utc;
use domain::{Contact, SnapshotId};
use ports::ContactRepositoryPort;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};
use std::sync::Arc;

use queries::{map_row_to_contact, FULL_CONTACT_SELECT};
use save::ContactSaveHelper;

pub struct ContactRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl ContactRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl ContactRepositoryPort for ContactRepository {
    fn save_contact(&self, snapshot_id: &SnapshotId, contact: &Contact) -> anyhow::Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        let existing_id: Option<String> = if let Some(hash) = &contact.content_hash {
            tx.query_row(
                "SELECT id FROM contact_objects WHERE content_hash = ?1",
                params![hash],
                |row| row.get(0),
            )
            .optional()?
        } else {
            None
        };

        let db_id = if let Some(id) = existing_id {
            id
        } else {
            let new_id = uuid::Uuid::new_v4().to_string();
            let created_at = Utc::now().to_rfc3339();

            tx.execute(
                "INSERT INTO contact_objects (id, source_id, display_name, notes, source, source_account, content_hash, metadata_json, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    new_id, contact.source_id, contact.display_name,
                    contact.notes, contact.source, contact.source_account,
                    contact.content_hash, contact.metadata_json, created_at
                ],
            )?;

            ContactSaveHelper::insert_contact_details(&tx, &new_id, contact)?;
            new_id
        };

        tx.execute(
            "INSERT OR IGNORE INTO snapshot_contacts (snapshot_id, contact_id) VALUES (?1, ?2)",
            params![snapshot_id.0, db_id],
        )?;

        for label_name in &contact.labels {
            let label_id = uuid::Uuid::new_v4().to_string();
            tx.execute(
                "INSERT INTO contact_labels (id, snapshot_id, name, source, source_account)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    label_id,
                    snapshot_id.0,
                    label_name,
                    contact.source,
                    contact.source_account
                ],
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
        let query = format!(
            "{} JOIN snapshot_contacts sc ON c.id = sc.contact_id WHERE sc.snapshot_id = ?1",
            FULL_CONTACT_SELECT
        );
        let mut stmt = conn.prepare(&query)?;
        let contacts = stmt
            .query_map([&snapshot_id.0], map_row_to_contact)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(contacts)
    }

    fn search_contacts(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, Contact)>> {
        let conn = self.pool.get()?;
        let fts_query = format!("\"{}\"*", query.replace("\"", "\"\""));
        let sql = format!(
            "{}
            JOIN snapshot_contacts sc ON c.id = sc.contact_id
            JOIN contacts_fts fts ON c.rowid = fts.rowid
            WHERE contacts_fts MATCH ?1 ORDER BY rank",
            FULL_CONTACT_SELECT
        );

        let mut stmt = conn.prepare(&sql)?;
        let results: Vec<(SnapshotId, Contact)> = stmt
            .query_map([fts_query], |row| {
                let snap_id: String = row.get_ref(0).unwrap().as_str().unwrap().to_string();
                Ok((SnapshotId(snap_id), map_row_to_contact(row)?))
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(results)
    }

    fn get_contact_diff(
        &self,
        old_snapshot_id: &SnapshotId,
        new_snapshot_id: &SnapshotId,
    ) -> anyhow::Result<domain::ContactDiff> {
        let old_contacts = self.get_snapshot_contacts(old_snapshot_id)?;
        let new_contacts = self.get_snapshot_contacts(new_snapshot_id)?;
        let mut diff = domain::ContactDiff::default();

        let old_map: std::collections::HashMap<String, Contact> = old_contacts
            .into_iter()
            .map(|c| (c.source_id.clone().unwrap_or_else(|| c.display_name.clone()), c))
            .collect();

        let mut new_map: std::collections::HashMap<String, Contact> = new_contacts
            .into_iter()
            .map(|c| (c.source_id.clone().unwrap_or_else(|| c.display_name.clone()), c))
            .collect();

        for (key, new_contact) in new_map.drain() {
            if let Some(old_contact) = old_map.get(&key) {
                if old_contact.content_hash != new_contact.content_hash {
                    diff.modified.push(new_contact);
                }
            } else {
                diff.added.push(new_contact);
            }
        }

        let new_contacts_again = self.get_snapshot_contacts(new_snapshot_id)?;
        let new_keys: std::collections::HashSet<String> = new_contacts_again
            .into_iter()
            .map(|c| c.source_id.unwrap_or(c.display_name))
            .collect();

        for (key, old_contact) in old_map {
            if !new_keys.contains(&key) {
                diff.removed.push(old_contact);
            }
        }

        Ok(diff)
    }
}
