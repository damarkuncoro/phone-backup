CREATE TABLE IF NOT EXISTS devices (
    id TEXT PRIMARY KEY,
    manufacturer TEXT NOT NULL,
    model TEXT NOT NULL,
    serial TEXT NOT NULL,
    os_version TEXT NOT NULL,
    storage_total_bytes INTEGER NOT NULL,
    storage_used_bytes INTEGER NOT NULL,
    connection_type TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS schedules (
    device_id TEXT PRIMARY KEY,
    frequency TEXT NOT NULL,
    last_run_at TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY(device_id) REFERENCES devices(id) ON DELETE CASCADE
);
