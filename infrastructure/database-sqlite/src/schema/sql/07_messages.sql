-- Messages (SMS/MMS)
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    snapshot_id TEXT NOT NULL,
    address TEXT NOT NULL,
    body TEXT NOT NULL,
    date TEXT NOT NULL,
    type_code INTEGER NOT NULL, -- 1: inbox, 2: sent, etc.
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_messages_snapshot_id ON messages(snapshot_id);
CREATE INDEX IF NOT EXISTS idx_messages_address ON messages(address);

-- Full-Text Search for Messages
CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
    id UNINDEXED,
    body,
    content='messages',
    content_rowid='rowid'
);

CREATE TRIGGER IF NOT EXISTS messages_ai AFTER INSERT ON messages BEGIN
  INSERT INTO messages_fts(rowid, id, body) VALUES (new.rowid, new.id, new.body);
END;

CREATE TRIGGER IF NOT EXISTS messages_ad AFTER DELETE ON messages BEGIN
  INSERT INTO messages_fts(messages_fts, rowid, id, body) VALUES('delete', old.rowid, old.id, old.body);
END;
