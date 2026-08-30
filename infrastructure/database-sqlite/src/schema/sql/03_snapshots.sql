CREATE TABLE IF NOT EXISTS snapshots (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    started_at TEXT NOT NULL,
    finished_at TEXT,
    status TEXT NOT NULL,
    total_files INTEGER NOT NULL DEFAULT 0,
    total_bytes INTEGER NOT NULL DEFAULT 0,
    deduped_bytes INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(device_id) REFERENCES devices(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_snapshots_device_id ON snapshots(device_id);

CREATE TABLE IF NOT EXISTS snapshot_files (
    snapshot_id TEXT NOT NULL,
    file_id TEXT NOT NULL,
    PRIMARY KEY(snapshot_id, file_id),
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE,
    FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS snapshot_data (
    snapshot_id TEXT NOT NULL,
    data_type TEXT NOT NULL,
    object_id TEXT NOT NULL,
    PRIMARY KEY(snapshot_id, data_type, object_id),
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE
);
