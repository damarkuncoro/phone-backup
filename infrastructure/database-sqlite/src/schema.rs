use rusqlite::Connection;

pub fn init_schema(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS devices (
            id TEXT PRIMARY KEY,
            manufacturer TEXT NOT NULL,
            model TEXT NOT NULL,
            serial TEXT NOT NULL,
            os_version TEXT NOT NULL,
            storage_total_bytes INTEGER NOT NULL,
            storage_used_bytes INTEGER NOT NULL,
            connection_type TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id TEXT PRIMARY KEY,
            device_id TEXT NOT NULL,
            path TEXT NOT NULL,
            name TEXT NOT NULL,
            size_bytes INTEGER NOT NULL,
            modified_at TEXT NOT NULL,
            mime_type TEXT NOT NULL,
            permissions TEXT NOT NULL,
            hash_sha256 TEXT,
            media_info TEXT,
            FOREIGN KEY(device_id) REFERENCES devices(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS snapshots (
            id TEXT PRIMARY KEY,
            device_id TEXT NOT NULL,
            started_at TEXT NOT NULL,
            finished_at TEXT,
            status TEXT NOT NULL,
            total_files INTEGER NOT NULL,
            total_bytes INTEGER NOT NULL,
            deduped_bytes INTEGER DEFAULT 0,
            FOREIGN KEY(device_id) REFERENCES devices(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS snapshot_files (
            snapshot_id TEXT NOT NULL,
            file_id TEXT NOT NULL,
            PRIMARY KEY(snapshot_id, file_id),
            FOREIGN KEY(snapshot_id) REFERENCES snapshots(id),
            FOREIGN KEY(file_id) REFERENCES files(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS apps (
            id TEXT PRIMARY KEY,
            device_id TEXT NOT NULL,
            package_name TEXT NOT NULL,
            version_name TEXT NOT NULL,
            version_code INTEGER NOT NULL,
            installer TEXT,
            app_name TEXT NOT NULL,
            FOREIGN KEY(device_id) REFERENCES devices(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS snapshot_apps (
            snapshot_id TEXT NOT NULL,
            app_id TEXT NOT NULL,
            PRIMARY KEY(snapshot_id, app_id),
            FOREIGN KEY(snapshot_id) REFERENCES snapshots(id),
            FOREIGN KEY(app_id) REFERENCES apps(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS snapshot_data (
            snapshot_id TEXT NOT NULL,
            data_type TEXT NOT NULL,
            object_id TEXT NOT NULL,
            PRIMARY KEY(snapshot_id, data_type),
            FOREIGN KEY(snapshot_id) REFERENCES snapshots(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS schedules (
            device_id TEXT PRIMARY KEY,
            frequency TEXT NOT NULL,
            last_run_at TEXT,
            enabled INTEGER NOT NULL,
            FOREIGN KEY(device_id) REFERENCES devices(id)
        )",
        [],
    )?;
    Ok(())
}
