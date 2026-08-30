CREATE TABLE IF NOT EXISTS contacts (
    id TEXT PRIMARY KEY,
    snapshot_id TEXT NOT NULL,
    source_id TEXT,
    display_name TEXT NOT NULL,
    notes TEXT,
    source TEXT NOT NULL DEFAULT 'unknown',
    source_account TEXT,
    content_hash TEXT,
    metadata_json TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contacts_snapshot_id ON contacts(snapshot_id);
CREATE INDEX IF NOT EXISTS idx_contacts_display_name ON contacts(display_name);
CREATE INDEX IF NOT EXISTS idx_contacts_source ON contacts(source, source_id);
CREATE INDEX IF NOT EXISTS idx_contacts_content_hash ON contacts(content_hash);

CREATE TABLE IF NOT EXISTS contact_names (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL UNIQUE,
    display_name TEXT,
    given_name TEXT,
    middle_name TEXT,
    family_name TEXT,
    prefix TEXT,
    suffix TEXT,
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contact_names_contact_id ON contact_names(contact_id);

CREATE TABLE IF NOT EXISTS contact_phones (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    raw_value TEXT NOT NULL,
    normalized_value TEXT,
    type TEXT,
    label TEXT,
    is_primary INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contact_phones_contact_id ON contact_phones(contact_id);
CREATE INDEX IF NOT EXISTS idx_contact_phones_normalized_value ON contact_phones(normalized_value);
CREATE UNIQUE INDEX IF NOT EXISTS idx_one_primary_phone ON contact_phones(contact_id) WHERE is_primary = 1;

CREATE TABLE IF NOT EXISTS contact_emails (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    value TEXT NOT NULL,
    type TEXT,
    label TEXT,
    is_primary INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contact_emails_contact_id ON contact_emails(contact_id);
CREATE INDEX IF NOT EXISTS idx_contact_emails_value ON contact_emails(value);
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
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contact_addresses_contact_id ON contact_addresses(contact_id);

CREATE TABLE IF NOT EXISTS contact_organizations (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    company_name TEXT,
    department TEXT,
    title TEXT,
    job_description TEXT,
    type TEXT,
    label TEXT,
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contact_organizations_contact_id ON contact_organizations(contact_id);

CREATE TABLE IF NOT EXISTS contact_urls (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    url TEXT NOT NULL,
    type TEXT,
    label TEXT,
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contact_urls_contact_id ON contact_urls(contact_id);

CREATE TABLE IF NOT EXISTS contact_events (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    event_date TEXT NOT NULL,
    label TEXT,
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contact_events_contact_id ON contact_events(contact_id);

CREATE TABLE IF NOT EXISTS contact_photos (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    file_id TEXT,
    photo_hash TEXT,
    mime_type TEXT,
    is_primary INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE,
    FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS contact_labels (
    id TEXT PRIMARY KEY,
    snapshot_id TEXT NOT NULL,
    name TEXT NOT NULL,
    source TEXT,
    source_account TEXT,
    source_id TEXT,
    FOREIGN KEY(snapshot_id) REFERENCES snapshots(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_contact_labels_snapshot_id ON contact_labels(snapshot_id);

CREATE TABLE IF NOT EXISTS contact_label_members (
    contact_id TEXT NOT NULL,
    label_id TEXT NOT NULL,
    PRIMARY KEY(contact_id, label_id),
    FOREIGN KEY(contact_id) REFERENCES contacts(id) ON DELETE CASCADE,
    FOREIGN KEY(label_id) REFERENCES contact_labels(id) ON DELETE CASCADE
);

-- =========================================================
-- FULL-TEXT SEARCH (FTS5) for Contacts
-- =========================================================
CREATE VIRTUAL TABLE IF NOT EXISTS contacts_fts USING fts5(
    id UNINDEXED,
    display_name,
    notes,
    content='contacts',
    content_rowid='rowid'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS contacts_ai AFTER INSERT ON contacts BEGIN
  INSERT INTO contacts_fts(rowid, id, display_name, notes) VALUES (new.rowid, new.id, new.display_name, new.notes);
END;

CREATE TRIGGER IF NOT EXISTS contacts_ad AFTER DELETE ON contacts BEGIN
  INSERT INTO contacts_fts(contacts_fts, rowid, id, display_name, notes) VALUES('delete', old.rowid, old.id, old.display_name, old.notes);
END;

CREATE TRIGGER IF NOT EXISTS contacts_au AFTER UPDATE ON contacts BEGIN
  INSERT INTO contacts_fts(contacts_fts, rowid, id, display_name, notes) VALUES('delete', old.rowid, old.id, old.display_name, old.notes);
  INSERT INTO contacts_fts(rowid, id, display_name, notes) VALUES (new.rowid, new.id, new.display_name, new.notes);
END;
