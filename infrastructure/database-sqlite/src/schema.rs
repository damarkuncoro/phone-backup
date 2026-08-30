use rusqlite::Connection;
use tracing::{info, debug};

pub fn init_schema(conn: &Connection) -> anyhow::Result<()> {
    // Create migrations table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // List of all migration scripts in order
    // In a real-world scenario, you might want to use a more sophisticated migration tool,
    // but this simple sequence-based approach is robust for our needs.
    let migrations = [
        (1, "Initial Schema", vec![
            include_str!("schema/sql/00_base.sql"),
            include_str!("schema/sql/01_devices.sql"),
            include_str!("schema/sql/02_files.sql"),
            include_str!("schema/sql/03_snapshots.sql"),
            include_str!("schema/sql/04_apps.sql"),
            include_str!("schema/sql/05_contacts.sql"),
            include_str!("schema/sql/06_settings.sql"),
        ]),
        // Future migrations go here:
        // (2, "Add email to devices", vec!["ALTER TABLE devices ADD COLUMN email TEXT"]),
        (2, "Add Full-Text Search for files and contacts", vec![
            "CREATE VIRTUAL TABLE IF NOT EXISTS files_fts USING fts5(id UNINDEXED, name, path, content='files', content_rowid='rowid');",
            "INSERT INTO files_fts(rowid, id, name, path) SELECT rowid, id, name, path FROM files;",
            "CREATE TRIGGER IF NOT EXISTS files_ai AFTER INSERT ON files BEGIN INSERT INTO files_fts(rowid, id, name, path) VALUES (new.rowid, new.id, new.name, new.path); END;",
            "CREATE TRIGGER IF NOT EXISTS files_ad AFTER DELETE ON files BEGIN INSERT INTO files_fts(files_fts, rowid, id, name, path) VALUES('delete', old.rowid, old.id, old.name, old.path); END;",
            "CREATE TRIGGER IF NOT EXISTS files_au AFTER UPDATE ON files BEGIN INSERT INTO files_fts(files_fts, rowid, id, name, path) VALUES('delete', old.rowid, old.id, old.name, old.path); INSERT INTO files_fts(rowid, id, name, path) VALUES (new.rowid, new.id, new.name, new.path); END;",
            "CREATE VIRTUAL TABLE IF NOT EXISTS contacts_fts USING fts5(id UNINDEXED, display_name, notes, content='contact_objects', content_rowid='rowid');",
            "INSERT INTO contacts_fts(rowid, id, display_name, notes) SELECT rowid, id, display_name, notes FROM contact_objects;",
            "CREATE TRIGGER IF NOT EXISTS contacts_ai AFTER INSERT ON contact_objects BEGIN INSERT INTO contacts_fts(rowid, id, display_name, notes) VALUES (new.rowid, new.id, new.display_name, new.notes); END;",
            "CREATE TRIGGER IF NOT EXISTS contacts_ad AFTER DELETE ON contact_objects BEGIN INSERT INTO contacts_fts(contacts_fts, rowid, id, display_name, notes) VALUES('delete', old.rowid, old.id, old.display_name, old.notes); END;",
            "CREATE TRIGGER IF NOT EXISTS contacts_au AFTER UPDATE ON contact_objects BEGIN INSERT INTO contacts_fts(contacts_fts, rowid, id, display_name, notes) VALUES('delete', old.rowid, old.id, old.display_name, old.notes); INSERT INTO contacts_fts(rowid, id, display_name, notes) VALUES (new.rowid, new.id, new.display_name, new.notes); END;",
        ]),
        (3, "Add SMS and Call Log tables", vec![
            include_str!("schema/sql/07_messages.sql"),
            include_str!("schema/sql/08_call_logs.sql"),
        ]),
        (4, "Add thumbnail_hash to files", vec![
            "ALTER TABLE files ADD COLUMN thumbnail_hash TEXT",
        ]),
    ];

    for (version, description, scripts) in migrations {
        if !is_migration_applied(conn, version)? {
            info!("Applying migration v{} ({})", version, description);

            // Run all scripts in this migration version as a single batch/transaction
            for script in scripts {
                conn.execute_batch(script)?;
            }

            conn.execute(
                "INSERT INTO _schema_migrations (version) VALUES (?)",
                [version],
            )?;
        } else {
            debug!("Migration v{} already applied", version);
        }
    }

    Ok(())
}

fn is_migration_applied(conn: &Connection, version: i32) -> anyhow::Result<bool> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM _schema_migrations WHERE version = ?",
        [version],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}
