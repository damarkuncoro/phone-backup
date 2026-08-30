CREATE TABLE IF NOT EXISTS apps (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    package_name TEXT NOT NULL,
    version_name TEXT NOT NULL,
    version_code INTEGER NOT NULL,
    installer TEXT,
    app_name TEXT NOT NULL,
    FOREIGN KEY(device_id) REFERENCES devices(id) ON DELETE CASCADE,
    UNIQUE(device_id, package_name, version_code)
);

CREATE INDEX IF NOT EXISTS idx_apps_device_id ON apps(device_id);

CREATE TABLE IF NOT EXISTS snapshot_apps (
    snapshot_id TEXT NOT NULL,
    app_id TEXT NOT NULL,
    PRIMARY KEY(snapshot_id, app_id),
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE,
    FOREIGN KEY(app_id) REFERENCES apps(id) ON DELETE CASCADE
);
