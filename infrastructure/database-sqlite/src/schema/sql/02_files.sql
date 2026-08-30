CREATE TABLE IF NOT EXISTS files (
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
    FOREIGN KEY(device_id) REFERENCES devices(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_files_device_id ON files(device_id);
CREATE INDEX IF NOT EXISTS idx_files_hash ON files(hash_sha256);

CREATE TABLE IF NOT EXISTS file_chunks (
    file_id TEXT NOT NULL,
    chunk_hash TEXT NOT NULL,
    chunk_offset INTEGER NOT NULL,
    chunk_length INTEGER NOT NULL,
    sequence INTEGER NOT NULL,
    PRIMARY KEY(file_id, sequence),
    FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_file_chunks_hash ON file_chunks(chunk_hash);

-- =========================================================
-- FULL-TEXT SEARCH (FTS5) for Files
-- =========================================================
CREATE VIRTUAL TABLE IF NOT EXISTS files_fts USING fts5(
    id UNINDEXED,
    name,
    path,
    content='files',
    content_rowid='rowid'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS files_ai AFTER INSERT ON files BEGIN
  INSERT INTO files_fts(rowid, id, name, path) VALUES (new.rowid, new.id, new.name, new.path);
END;

CREATE TRIGGER IF NOT EXISTS files_ad AFTER DELETE ON files BEGIN
  INSERT INTO files_fts(files_fts, rowid, id, name, path) VALUES('delete', old.rowid, old.id, old.name, old.path);
END;

CREATE TRIGGER IF NOT EXISTS files_au AFTER UPDATE ON files BEGIN
  INSERT INTO files_fts(files_fts, rowid, id, name, path) VALUES('delete', old.rowid, old.id, old.name, old.path);
  INSERT INTO files_fts(rowid, id, name, path) VALUES (new.rowid, new.id, new.name, new.path);
END;
