mod mappers;
mod schema;

use crate::mappers::RowMapper;
use domain::{AppId, AppInfo, BackupSchedule, Device, DeviceId, FileEntry, Snapshot, SnapshotId, Contact};
use ports::RepositoryPort;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use tracing::instrument;

pub struct SqliteRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteRepository {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(path)?;
        let repo = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        repo.init_db()?;
        Ok(repo)
    }

    fn init_db(&self) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        schema::init_schema(&conn)?;
        Ok(())
    }
}

impl RepositoryPort for SqliteRepository {
    #[instrument(skip(self))]
    fn save_device(&self, device: &Device) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO devices
            (id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                device.id.0, device.manufacturer, device.model, device.serial,
                device.os_version, device.storage_total_bytes, device.storage_used_bytes,
                format!("{:?}", device.connection_type)
            ],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn save_file(&self, file: &FileEntry) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let media_info_json = file.media_info.as_ref().map(|m| serde_json::to_string(m).unwrap());
        conn.execute(
            "INSERT OR REPLACE INTO files
            (id, device_id, path, name, size_bytes, modified_at, mime_type, permissions, hash_sha256, media_info)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                file.id.0, file.device_id.0, file.path, file.name, file.size_bytes,
                file.modified_at.to_rfc3339(), file.mime_type, file.permissions,
                file.hash_sha256, media_info_json
            ],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, manufacturer, model, serial, os_version, storage_total_bytes, storage_used_bytes, connection_type FROM devices")?;
        let device_iter = stmt.query_map([], RowMapper::to_device)?;
        let mut devices = Vec::new();
        for d in device_iter { devices.push(d?); }
        Ok(devices)
    }

    #[instrument(skip(self))]
    fn get_device(&self, _id: &DeviceId) -> anyhow::Result<Option<Device>> {
        Ok(None)
    }

    #[instrument(skip(self))]
    fn list_files(&self, device_id: &DeviceId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM files WHERE device_id = ?1")?;
        let file_iter = stmt.query_map([&device_id.0], RowMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }

    #[instrument(skip(self))]
    fn create_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO snapshots (id, device_id, started_at, finished_at, status, total_files, total_bytes, deduped_bytes)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                snapshot.id.0, snapshot.device_id.0, snapshot.started_at.to_rfc3339(),
                snapshot.finished_at.map(|t| t.to_rfc3339()), format!("{:?}", snapshot.status),
                snapshot.total_files, snapshot.total_bytes, snapshot.deduped_bytes
            ],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn update_snapshot(&self, snapshot: &Snapshot) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE snapshots SET finished_at = ?2, status = ?3, total_files = ?4, total_bytes = ?5, deduped_bytes = ?6
            WHERE id = ?1",
            params![
                snapshot.id.0, snapshot.finished_at.map(|t| t.to_rfc3339()),
                format!("{:?}", snapshot.status), snapshot.total_files,
                snapshot.total_bytes, snapshot.deduped_bytes
            ],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn get_snapshot(&self, id: &SnapshotId) -> anyhow::Result<Option<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM snapshots WHERE id = ?1")?;
        let mut snapshot_iter = stmt.query_map([&id.0], RowMapper::to_snapshot)?;
        if let Some(s) = snapshot_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    #[instrument(skip(self))]
    fn link_file_to_snapshot(&self, snapshot_id: &SnapshotId, file_id: &domain::FileId) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_files (snapshot_id, file_id) VALUES (?1, ?2)",
            params![snapshot_id.0, file_id.0],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn list_snapshots(&self, device_id: &DeviceId) -> anyhow::Result<Vec<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM snapshots WHERE device_id = ?1 ORDER BY started_at DESC")?;
        let snapshot_iter = stmt.query_map([&device_id.0], RowMapper::to_snapshot)?;
        let mut snapshots = Vec::new();
        for s in snapshot_iter { snapshots.push(s?); }
        Ok(snapshots)
    }

    #[instrument(skip(self))]
    fn list_all_snapshots(&self) -> anyhow::Result<Vec<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM snapshots ORDER BY started_at DESC")?;
        let snapshot_iter = stmt.query_map([], RowMapper::to_snapshot)?;
        let mut snapshots = Vec::new();
        for s in snapshot_iter { snapshots.push(s?); }
        Ok(snapshots)
    }

    #[instrument(skip(self))]
    fn get_latest_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let snapshots = self.list_snapshots(device_id)?;
        Ok(snapshots.into_iter().find(|s| s.status == domain::SnapshotStatus::Completed))
    }

    #[instrument(skip(self))]
    fn get_incomplete_snapshot(&self, device_id: &DeviceId) -> anyhow::Result<Option<Snapshot>> {
        let snapshots = self.list_snapshots(device_id)?;
        Ok(snapshots.into_iter().find(|s| s.status == domain::SnapshotStatus::Running || s.status == domain::SnapshotStatus::Interrupted))
    }

    #[instrument(skip(self))]
    fn get_snapshot_files(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT f.* FROM files f JOIN snapshot_files sf ON f.id = sf.file_id WHERE sf.snapshot_id = ?1"
        )?;
        let file_iter = stmt.query_map([&snapshot_id.0], RowMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }

    #[instrument(skip(self))]
    fn save_app(&self, app: &AppInfo) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO apps (id, device_id, package_name, version_name, version_code, installer, app_name)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![app.id.0, app.device_id.0, app.package_name, app.version_name, app.version_code, app.installer, app.app_name],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn link_app_to_snapshot(&self, snapshot_id: &SnapshotId, app_id: &AppId) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO snapshot_apps (snapshot_id, app_id) VALUES (?1, ?2)",
            params![snapshot_id.0, app_id.0],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn get_snapshot_apps(&self, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<AppInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT a.* FROM apps a JOIN snapshot_apps sa ON a.id = sa.app_id WHERE sa.snapshot_id = ?1"
        )?;
        let app_iter = stmt.query_map([&snapshot_id.0], RowMapper::to_app)?;
        let mut apps = Vec::new();
        for a in app_iter { apps.push(a?); }
        Ok(apps)
    }

    #[instrument(skip(self))]
    fn save_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str, object_id: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO snapshot_data (snapshot_id, data_type, object_id) VALUES (?1, ?2, ?3)",
            params![snapshot_id.0, data_type, object_id],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn get_structured_data_ref(&self, snapshot_id: &SnapshotId, data_type: &str) -> anyhow::Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT object_id FROM snapshot_data WHERE snapshot_id = ?1 AND data_type = ?2"
        )?;
        let mut rows = stmt.query(params![snapshot_id.0, data_type])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    #[instrument(skip(self))]
    fn save_schedule(&self, schedule: &BackupSchedule) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO schedules (device_id, frequency, last_run_at, enabled) VALUES (?1, ?2, ?3, ?4)",
            params![schedule.device_id.0, format!("{:?}", schedule.frequency), schedule.last_run_at.map(|t| t.to_rfc3339()), if schedule.enabled { 1 } else { 0 }],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn get_schedule(&self, device_id: &DeviceId) -> anyhow::Result<Option<BackupSchedule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM schedules WHERE device_id = ?1")?;
        let mut schedule_iter = stmt.query_map([&device_id.0], RowMapper::to_schedule)?;
        if let Some(s) = schedule_iter.next() { Ok(Some(s?)) } else { Ok(None) }
    }

    #[instrument(skip(self))]
    fn list_schedules(&self) -> anyhow::Result<Vec<BackupSchedule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM schedules WHERE enabled = 1")?;
        let schedule_iter = stmt.query_map([], RowMapper::to_schedule)?;
        let mut schedules = Vec::new();
        for s in schedule_iter { schedules.push(s?); }
        Ok(schedules)
    }

    #[instrument(skip(self))]
    fn delete_snapshot(&self, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM snapshot_files WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        tx.execute("DELETE FROM snapshot_apps WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        tx.execute("DELETE FROM snapshot_data WHERE snapshot_id = ?1", [&snapshot_id.0])?;
        tx.execute("DELETE FROM snapshots WHERE id = ?1", [&snapshot_id.0])?;
        tx.commit()?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn search_files(&self, query: &str) -> anyhow::Result<Vec<FileEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM files WHERE name LIKE ?1 OR path LIKE ?1")?;
        let pattern = format!("%{}%", query);
        let file_iter = stmt.query_map([pattern], RowMapper::to_file)?;
        let mut files = Vec::new();
        for f in file_iter { files.push(f?); }
        Ok(files)
    }

    #[instrument(skip(self))]
    fn save_contact(&self, snapshot_id: &SnapshotId, contact: &domain::Contact) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        let created_at = chrono::Utc::now().to_rfc3339();

        // 1. Save main contact entry
        tx.execute(
            "INSERT INTO contacts (id, snapshot_id, source_id, display_name, notes, source, source_account, content_hash, metadata_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                contact.id, snapshot_id.0, contact.source_id, contact.display_name,
                contact.notes, contact.source, contact.source_account,
                contact.content_hash, contact.metadata_json, created_at
            ],
        )?;

        // 2. Save names
        for name in &contact.names {
            tx.execute(
                "INSERT INTO contact_names (id, contact_id, display_name, given_name, middle_name, family_name, prefix, suffix)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    uuid::Uuid::new_v4().to_string(), contact.id,
                    name.display_name, name.given_name, name.middle_name,
                    name.family_name, name.prefix, name.suffix
                ],
            )?;
        }

        // 3. Save phones
        for phone in &contact.phones {
            tx.execute(
                "INSERT INTO contact_phones (id, contact_id, raw_value, normalized_value, type, label, is_primary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    uuid::Uuid::new_v4().to_string(), contact.id,
                    phone.raw_value, phone.normalized_value, phone.phone_type,
                    phone.label, if phone.is_primary { 1 } else { 0 }
                ],
            )?;
        }

        // 4. Save emails
        for email in &contact.emails {
            tx.execute(
                "INSERT INTO contact_emails (id, contact_id, value, type, label, is_primary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    uuid::Uuid::new_v4().to_string(), contact.id,
                    email.value, email.email_type, email.label,
                    if email.is_primary { 1 } else { 0 }
                ],
            )?;
        }

        // 5. Save addresses
        for addr in &contact.addresses {
            tx.execute(
                "INSERT INTO contact_addresses (id, contact_id, formatted_address, street, city, region, postal_code, country, country_code, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    uuid::Uuid::new_v4().to_string(), contact.id,
                    addr.formatted_address, addr.street, addr.city, addr.region,
                    addr.postal_code, addr.country, addr.country_code,
                    addr.address_type, addr.label
                ],
            )?;
        }

        // 6. Save organizations
        for org in &contact.organizations {
            tx.execute(
                "INSERT INTO contact_organizations (id, contact_id, company_name, department, title, job_description, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    uuid::Uuid::new_v4().to_string(), contact.id,
                    org.company_name, org.department, org.title,
                    org.job_description, org.org_type, org.label
                ],
            )?;
        }

        // 7. Save URLs
        for url in &contact.urls {
            tx.execute(
                "INSERT INTO contact_urls (id, contact_id, url, type, label)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![uuid::Uuid::new_v4().to_string(), contact.id, url.url, url.url_type, url.label],
            )?;
        }

        // 8. Save Events
        for event in &contact.events {
            tx.execute(
                "INSERT INTO contact_events (id, contact_id, event_type, event_date, label)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![uuid::Uuid::new_v4().to_string(), contact.id, event.event_type, event.event_date, event.label],
            )?;
        }

        // 9. Save Photos
        for photo in &contact.photos {
            tx.execute(
                "INSERT INTO contact_photos (id, contact_id, file_id, photo_hash, mime_type, is_primary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    uuid::Uuid::new_v4().to_string(), contact.id,
                    photo.file_id, photo.photo_hash, photo.mime_type,
                    if photo.is_primary { 1 } else { 0 }
                ],
            )?;
        }

        // 10. Save Labels
        for label_name in &contact.labels {
            let label_id = uuid::Uuid::new_v4().to_string();
            tx.execute(
                "INSERT INTO contact_labels (id, snapshot_id, name, source, source_account)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![label_id, snapshot_id.0, label_name, contact.source, contact.source_account],
            )?;

            tx.execute(
                "INSERT INTO contact_label_members (contact_id, label_id) VALUES (?1, ?2)",
                params![contact.id, label_id],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn search_contacts(&self, query: &str) -> anyhow::Result<Vec<(SnapshotId, domain::Contact)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM contacts WHERE display_name LIKE ?1"
        )?;
        let pattern = format!("%{}%", query);

        let contact_rows: Vec<(String, SnapshotId, Option<String>, String, Option<String>, String, Option<String>, Option<String>, Option<String>)> = stmt.query_map([pattern], |row| {
            Ok((
                row.get::<_, String>(0)?,
                SnapshotId(row.get::<_, String>(1)?),
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
            ))
        })?.collect::<Result<Vec<_>, _>>()?;

        let mut results = Vec::new();
        for (id, s_id, source_id, display_name, notes, source, source_account, content_hash, metadata_json) in contact_rows {
            let names = conn.prepare("SELECT * FROM contact_names WHERE contact_id = ?1")?
                .query_map([&id], RowMapper::to_contact_name)?.collect::<Result<Vec<_>, _>>()?;

            let phones = conn.prepare("SELECT * FROM contact_phones WHERE contact_id = ?1")?
                .query_map([&id], RowMapper::to_contact_phone)?.collect::<Result<Vec<_>, _>>()?;

            let emails = conn.prepare("SELECT * FROM contact_emails WHERE contact_id = ?1")?
                .query_map([&id], RowMapper::to_contact_email)?.collect::<Result<Vec<_>, _>>()?;

            let addresses = conn.prepare("SELECT * FROM contact_addresses WHERE contact_id = ?1")?
                .query_map([&id], RowMapper::to_contact_address)?.collect::<Result<Vec<_>, _>>()?;

            let organizations = conn.prepare("SELECT * FROM contact_organizations WHERE contact_id = ?1")?
                .query_map([&id], RowMapper::to_contact_organization)?.collect::<Result<Vec<_>, _>>()?;

            let urls = conn.prepare("SELECT * FROM contact_urls WHERE contact_id = ?1")?
                .query_map([&id], RowMapper::to_contact_url)?.collect::<Result<Vec<_>, _>>()?;

            let events = conn.prepare("SELECT * FROM contact_events WHERE contact_id = ?1")?
                .query_map([&id], RowMapper::to_contact_event)?.collect::<Result<Vec<_>, _>>()?;

            let photos = conn.prepare("SELECT * FROM contact_photos WHERE contact_id = ?1")?
                .query_map([&id], RowMapper::to_contact_photo)?.collect::<Result<Vec<_>, _>>()?;

            let labels = conn.prepare(
                "SELECT cl.name FROM contact_labels cl
                 JOIN contact_label_members clm ON cl.id = clm.label_id
                 WHERE clm.contact_id = ?1"
            )?.query_map([&id], |row| row.get::<_, String>(0))?.collect::<Result<Vec<_>, _>>()?;

            results.push((s_id.clone(), Contact {
                id,
                snapshot_id: Some(s_id.0.clone()),
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
            }));
        }

        Ok(results)
    }

    #[instrument(skip(self))]
    fn save_file_chunk(&self, file_id: &domain::FileId, chunk_hash: &str, offset: u64, length: u32, sequence: u32) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO file_chunks (file_id, chunk_hash, chunk_offset, chunk_length, sequence)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![file_id.0, chunk_hash, offset, length, sequence],
        )?;
        Ok(())
    }

    #[instrument(skip(self))]
    fn get_file_chunks(&self, file_id: &domain::FileId) -> anyhow::Result<Vec<(String, u64, u32)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT chunk_hash, chunk_offset, chunk_length FROM file_chunks
             WHERE file_id = ?1 ORDER BY sequence ASC"
        )?;

        let chunk_iter = stmt.query_map([&file_id.0], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut chunks = Vec::new();
        for c in chunk_iter {
            chunks.push(c?);
        }
        Ok(chunks)
    }

    #[instrument(skip(self))]
    fn get_all_referenced_hashes(&self) -> anyhow::Result<std::collections::HashSet<String>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT hash_sha256 FROM files WHERE hash_sha256 IS NOT NULL
             UNION
             SELECT chunk_hash FROM file_chunks"
        )?;

        let hash_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut hashes = std::collections::HashSet::new();
        for h in hash_iter {
            hashes.insert(h?);
        }

        // Add hashes from structured data manually to be safe
        let mut stmt_data = conn.prepare("SELECT object_id FROM snapshot_data")?;
        let data_iter = stmt_data.query_map([], |row| row.get::<_, String>(0))?;
        for path in data_iter {
            let path = path?;
            if let Some(filename) = path.split('/').last() {
                let hash = filename.split('.').next().unwrap_or("");
                hashes.insert(hash.to_string());
            }
        }

        Ok(hashes)
    }
}
