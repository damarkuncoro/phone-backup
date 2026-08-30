-- Call Logs
CREATE TABLE IF NOT EXISTS call_logs (
    id TEXT PRIMARY KEY,
    snapshot_id TEXT NOT NULL,
    number TEXT NOT NULL,
    name TEXT,
    date TEXT NOT NULL,
    duration_seconds INTEGER NOT NULL,
    type_code INTEGER NOT NULL, -- 1: incoming, 2: outgoing, 3: missed, etc.
    location TEXT,
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_call_logs_snapshot_id ON call_logs(snapshot_id);
CREATE INDEX IF NOT EXISTS idx_call_logs_number ON call_logs(number);
