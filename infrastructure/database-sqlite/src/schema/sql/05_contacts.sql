-- Contact Objects (Deduplicated storage)
CREATE TABLE IF NOT EXISTS contact_objects (
    id TEXT PRIMARY KEY,
    source_id TEXT,
    display_name TEXT NOT NULL,
    notes TEXT,
    source TEXT NOT NULL DEFAULT 'unknown',
    source_account TEXT,
    content_hash TEXT UNIQUE, -- Used for deduplication
    metadata_json TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_contact_objects_display_name ON contact_objects(display_name);
CREATE INDEX IF NOT EXISTS idx_contact_objects_source ON contact_objects(source, source_id);

-- Junction table linking snapshots to contacts
CREATE TABLE IF NOT EXISTS snapshot_contacts (
    snapshot_id TEXT NOT NULL,
    contact_id TEXT NOT NULL,
    PRIMARY KEY(snapshot_id, contact_id),
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_snapshot_contacts_snapshot_id ON snapshot_contacts(snapshot_id);

-- Relational details linking to contact_objects
CREATE TABLE IF NOT EXISTS contact_names (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL UNIQUE,
    display_name TEXT,
    given_name TEXT,
    middle_name TEXT,
    family_name TEXT,
    prefix TEXT,
    suffix TEXT,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contact_phones (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    raw_value TEXT NOT NULL,
    normalized_value TEXT,
    type TEXT,
    label TEXT,
    is_primary INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_one_primary_phone ON contact_phones(contact_id) WHERE is_primary = 1;

CREATE TABLE IF NOT EXISTS contact_emails (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    value TEXT NOT NULL,
    type TEXT,
    label TEXT,
    is_primary INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_one_primary_email ON contact_emails(contact_id) WHERE is_primary = 1;

CREATE TABLE IF NOT EXISTS contact_addresses (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    formatted_address TEXT,
    street TEXT,
    city TEXT,
    region TEXT,
    postal_code TEXT,
    country TEXT,
    country_code TEXT,
    type TEXT,
    label TEXT,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contact_organizations (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    company_name TEXT,
    department TEXT,
    title TEXT,
    job_description TEXT,
    type TEXT,
    label TEXT,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contact_urls (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    url TEXT NOT NULL,
    type TEXT,
    label TEXT,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contact_events (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    event_date TEXT NOT NULL,
    label TEXT,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contact_photos (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    file_id TEXT,
    photo_hash TEXT,
    mime_type TEXT,
    is_primary INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE,
    FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE SET NULL
);

-- Labels are still per snapshot context as they often change/are user-defined per device state
CREATE TABLE IF NOT EXISTS contact_labels (
    id TEXT PRIMARY KEY,
    snapshot_id TEXT NOT NULL,
    name TEXT NOT NULL,
    source TEXT,
    source_account TEXT,
    source_id TEXT,
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contact_label_members (
    contact_id TEXT NOT NULL,
    label_id TEXT NOT NULL,
    PRIMARY KEY(contact_id, label_id),
    FOREIGN KEY(contact_id) REFERENCES contact_objects(id) ON DELETE CASCADE,
    FOREIGN KEY(label_id) REFERENCES contact_labels(id) ON DELETE CASCADE
);

-- =========================================================
-- FULL-TEXT SEARCH (FTS5) for Contacts
-- =========================================================
CREATE VIRTUAL TABLE IF NOT EXISTS contacts_fts USING fts5(
    id UNINDEXED,
    display_name,
    notes,
    content='contact_objects',
    content_rowid='rowid'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS contacts_ai AFTER INSERT ON contact_objects BEGIN
  INSERT INTO contacts_fts(rowid, id, display_name, notes) VALUES (new.rowid, new.id, new.display_name, new.notes);
END;

CREATE TRIGGER IF NOT EXISTS contacts_ad AFTER DELETE ON contact_objects BEGIN
  INSERT INTO contacts_fts(contacts_fts, rowid, id, display_name, notes) VALUES('delete', old.rowid, old.id, old.display_name, old.notes);
END;

CREATE TRIGGER IF NOT EXISTS contacts_au AFTER UPDATE ON contact_objects BEGIN
  INSERT INTO contacts_fts(contacts_fts, rowid, id, display_name, notes) VALUES('delete', old.rowid, old.id, old.display_name, old.notes);
  INSERT INTO contacts_fts(rowid, id, display_name, notes) VALUES (new.rowid, new.id, new.display_name, new.notes);
END;
