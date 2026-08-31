Schema terbaru Anda sudah **lebih baik dan lebih tepat** untuk backup contact dibanding versi sebelumnya. Relasi `contacts → phones/emails/addresses/organizations` sudah benar.

Namun, untuk target **backup HP + kompatibilitas Google Contacts + restore**, saya sarankan beberapa perbaikan penting:

1. Aktifkan SQLite foreign keys.
2. Tambahkan tabel nama terstruktur.
3. Tambahkan `updated_at`.
4. Tambahkan `normalized_value` untuk nomor telepon.
5. Tambahkan `label` custom.
6. Tambahkan index.
7. Perbaiki constraint `snapshot_data`.
8. Gunakan `execute_batch()` agar schema lebih mudah dirawat.

Berikut versi yang saya rekomendasikan.

use rusqlite::Connection;

pub fn init_schema(conn: &Connection) -> anyhow::Result<()> {
conn.execute_batch(
r#"
PRAGMA foreign_keys = ON;

```
    -- =========================================================
    -- DEVICES
    -- =========================================================

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

    -- =========================================================
    -- FILES
    -- =========================================================

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

        FOREIGN KEY(device_id)
            REFERENCES devices(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_files_device_id
    ON files(device_id);

    CREATE INDEX IF NOT EXISTS idx_files_hash
    ON files(hash_sha256);

    -- =========================================================
    -- SNAPSHOTS
    -- =========================================================

    CREATE TABLE IF NOT EXISTS snapshots (
        id TEXT PRIMARY KEY,

        device_id TEXT NOT NULL,

        started_at TEXT NOT NULL,

        finished_at TEXT,

        status TEXT NOT NULL,

        total_files INTEGER NOT NULL DEFAULT 0,

        total_bytes INTEGER NOT NULL DEFAULT 0,

        deduped_bytes INTEGER NOT NULL DEFAULT 0,

        FOREIGN KEY(device_id)
            REFERENCES devices(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_snapshots_device_id
    ON snapshots(device_id);

    -- =========================================================
    -- SNAPSHOT FILES
    -- =========================================================

    CREATE TABLE IF NOT EXISTS snapshot_files (
        snapshot_id TEXT NOT NULL,

        file_id TEXT NOT NULL,

        PRIMARY KEY(snapshot_id, file_id),

        FOREIGN KEY(snapshot_id)
            REFERENCES snapshots(id)
            ON DELETE CASCADE,

        FOREIGN KEY(file_id)
            REFERENCES files(id)
            ON DELETE CASCADE
    );

    -- =========================================================
    -- APPS
    -- =========================================================

    CREATE TABLE IF NOT EXISTS apps (
        id TEXT PRIMARY KEY,

        device_id TEXT NOT NULL,

        package_name TEXT NOT NULL,

        version_name TEXT NOT NULL,

        version_code INTEGER NOT NULL,

        installer TEXT,

        app_name TEXT NOT NULL,

        FOREIGN KEY(device_id)
            REFERENCES devices(id)
            ON DELETE CASCADE,

        UNIQUE(device_id, package_name, version_code)
    );

    CREATE INDEX IF NOT EXISTS idx_apps_device_id
    ON apps(device_id);

    -- =========================================================
    -- SNAPSHOT APPS
    -- =========================================================

    CREATE TABLE IF NOT EXISTS snapshot_apps (
        snapshot_id TEXT NOT NULL,

        app_id TEXT NOT NULL,

        PRIMARY KEY(snapshot_id, app_id),

        FOREIGN KEY(snapshot_id)
            REFERENCES snapshots(id)
            ON DELETE CASCADE,

        FOREIGN KEY(app_id)
            REFERENCES apps(id)
            ON DELETE CASCADE
    );

    -- =========================================================
    -- GENERIC SNAPSHOT DATA
    -- =========================================================

    CREATE TABLE IF NOT EXISTS snapshot_data (
        snapshot_id TEXT NOT NULL,

        data_type TEXT NOT NULL,

        object_id TEXT NOT NULL,

        PRIMARY KEY(snapshot_id, data_type, object_id),

        FOREIGN KEY(snapshot_id)
            REFERENCES snapshots(id)
            ON DELETE CASCADE
    );

    -- =========================================================
    -- FILE CHUNKS / DEDUPLICATION
    -- =========================================================

    CREATE TABLE IF NOT EXISTS file_chunks (
        file_id TEXT NOT NULL,

        chunk_hash TEXT NOT NULL,

        chunk_offset INTEGER NOT NULL,

        chunk_length INTEGER NOT NULL,

        sequence INTEGER NOT NULL,

        PRIMARY KEY(file_id, sequence),

        FOREIGN KEY(file_id)
            REFERENCES files(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_file_chunks_hash
    ON file_chunks(chunk_hash);

    -- =========================================================
    -- SCHEDULES
    -- =========================================================

    CREATE TABLE IF NOT EXISTS schedules (
        device_id TEXT PRIMARY KEY,

        frequency TEXT NOT NULL,

        last_run_at TEXT,

        enabled INTEGER NOT NULL DEFAULT 1,

        FOREIGN KEY(device_id)
            REFERENCES devices(id)
            ON DELETE CASCADE
    );

    -- =========================================================
    -- CONTACTS
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contacts (
        id TEXT PRIMARY KEY,

        snapshot_id TEXT NOT NULL,

        display_name TEXT NOT NULL,

        notes TEXT,

        source TEXT NOT NULL DEFAULT 'unknown',

        source_account TEXT,

        created_at TEXT NOT NULL,

        updated_at TEXT,

        FOREIGN KEY(snapshot_id)
            REFERENCES snapshots(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contacts_snapshot_id
    ON contacts(snapshot_id);

    CREATE INDEX IF NOT EXISTS idx_contacts_display_name
    ON contacts(display_name);

    -- =========================================================
    -- CONTACT NAMES
    -- Google Contacts compatible structured names
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_names (
        id TEXT PRIMARY KEY,

        contact_id TEXT NOT NULL,

        display_name TEXT,

        given_name TEXT,

        middle_name TEXT,

        family_name TEXT,

        prefix TEXT,

        suffix TEXT,

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contact_names_contact_id
    ON contact_names(contact_id);

    -- =========================================================
    -- CONTACT PHONES
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_phones (
        id TEXT PRIMARY KEY,

        contact_id TEXT NOT NULL,

        value TEXT NOT NULL,

        normalized_value TEXT,

        type TEXT,

        label TEXT,

        is_primary INTEGER NOT NULL DEFAULT 0,

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contact_phones_contact_id
    ON contact_phones(contact_id);

    CREATE INDEX IF NOT EXISTS idx_contact_phones_normalized_value
    ON contact_phones(normalized_value);

    -- =========================================================
    -- CONTACT EMAILS
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_emails (
        id TEXT PRIMARY KEY,

        contact_id TEXT NOT NULL,

        value TEXT NOT NULL,

        type TEXT,

        label TEXT,

        is_primary INTEGER NOT NULL DEFAULT 0,

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contact_emails_contact_id
    ON contact_emails(contact_id);

    CREATE INDEX IF NOT EXISTS idx_contact_emails_value
    ON contact_emails(value);

    -- =========================================================
    -- CONTACT ADDRESSES
    -- =========================================================

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

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contact_addresses_contact_id
    ON contact_addresses(contact_id);

    -- =========================================================
    -- CONTACT ORGANIZATIONS
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_organizations (
        id TEXT PRIMARY KEY,

        contact_id TEXT NOT NULL,

        company_name TEXT,

        department TEXT,

        title TEXT,

        job_description TEXT,

        type TEXT,

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contact_organizations_contact_id
    ON contact_organizations(contact_id);

    -- =========================================================
    -- CONTACT URLS
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_urls (
        id TEXT PRIMARY KEY,

        contact_id TEXT NOT NULL,

        url TEXT NOT NULL,

        type TEXT,

        label TEXT,

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contact_urls_contact_id
    ON contact_urls(contact_id);

    -- =========================================================
    -- CONTACT EVENTS
    -- Birthday / Anniversary / Custom Events
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_events (
        id TEXT PRIMARY KEY,

        contact_id TEXT NOT NULL,

        event_type TEXT NOT NULL,

        event_date TEXT NOT NULL,

        label TEXT,

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contact_events_contact_id
    ON contact_events(contact_id);

    -- =========================================================
    -- CONTACT PHOTOS
    -- Store only metadata/reference, not binary image
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_photos (
        id TEXT PRIMARY KEY,

        contact_id TEXT NOT NULL,

        file_id TEXT,

        mime_type TEXT,

        is_primary INTEGER NOT NULL DEFAULT 1,

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE,

        FOREIGN KEY(file_id)
            REFERENCES files(id)
            ON DELETE SET NULL
    );

    -- =========================================================
    -- CONTACT LABELS / GROUPS
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_labels (
        id TEXT PRIMARY KEY,

        snapshot_id TEXT NOT NULL,

        name TEXT NOT NULL,

        source_id TEXT,

        FOREIGN KEY(snapshot_id)
            REFERENCES snapshots(id)
            ON DELETE CASCADE
    );

    CREATE INDEX IF NOT EXISTS idx_contact_labels_snapshot_id
    ON contact_labels(snapshot_id);

    -- =========================================================
    -- CONTACT LABEL MEMBERS
    -- Many-to-Many
    -- =========================================================

    CREATE TABLE IF NOT EXISTS contact_label_members (
        contact_id TEXT NOT NULL,

        label_id TEXT NOT NULL,

        PRIMARY KEY(contact_id, label_id),

        FOREIGN KEY(contact_id)
            REFERENCES contacts(id)
            ON DELETE CASCADE,

        FOREIGN KEY(label_id)
            REFERENCES contact_labels(id)
            ON DELETE CASCADE
    );
    "#,
)?;

Ok(())
```

}

## Perubahan paling penting

### `contact_names`

Sebelumnya hanya:

```text
display_name
```

Sekarang dapat menyimpan:

```text
Damar Kuncoro

given_name  = Damar
middle_name = NULL
family_name = Kuncoro
prefix      = NULL
suffix      = NULL
```

Ini jauh lebih baik untuk proses restore ke Android atau export ke vCard.

---

### `normalized_value` pada nomor telepon

Contoh data asli:

```text
0812-1234-5678
+62 812 1234 5678
081212345678
```

Dapat dinormalisasi menjadi:

```text
+6281212345678
```

Ini penting untuk:

* Contact deduplication
* Contact merging
* Search
* Restore
* Deteksi duplicate contact

---

## Struktur akhirnya

```text
snapshots
    │
    └── contacts
          │
          ├── contact_names
          ├── contact_phones
          ├── contact_emails
          ├── contact_addresses
          ├── contact_organizations
          ├── contact_urls
          ├── contact_events
          ├── contact_photos
          │
          └── contact_label_members
                    │
                    ▼
              contact_labels
```

## Satu rekomendasi arsitektur tambahan

Untuk **backup system**, `contacts` yang terikat langsung ke `snapshot_id` sebenarnya sudah bagus untuk mempertahankan histori snapshot.

Contohnya:

```text
Snapshot 001
├── Damar
└── Budi

Snapshot 002
├── Damar
├── Budi
└── Andi
```

Dengan desain ini Anda bisa nanti membangun fitur:

```text
Snapshot Diff
```

Misalnya:

```text
+ Contact Added
+ Phone Number Added
~ Contact Modified
- Contact Deleted
```

Ini akan sangat cocok untuk roadmap berikutnya: **incremental contact backup + deduplication + restore engine**.
