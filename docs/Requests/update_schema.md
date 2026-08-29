Saya sudah meninjau schema lengkap yang Anda kirim. Secara struktur, ini sudah menjadi fondasi yang cukup baik untuk **Backup Engine HP berbasis Rust + SQLite**.

## Kesimpulan utama

Untuk bagian **Contact**, desain Anda sekarang sudah jauh lebih matang karena sudah mendukung:

* Multiple phone number
* Multiple email
* Structured name
* Address
* Organization
* URL
* Birthday/event
* Photo
* Label/group
* Source account
* Snapshot history

Bagian contact schema terdapat mulai tabel `contacts` dan relasi turunannya.

Namun, saya menemukan beberapa hal yang menurut saya perlu diperbaiki sebelum melanjutkan ke implementasi repository dan backup engine.

---

# 1. Masalah utama: `contact_names` bisa memiliki banyak row

Saat ini:

```sql
CREATE TABLE IF NOT EXISTS contact_names (
    id TEXT PRIMARY KEY,
    contact_id TEXT NOT NULL,
    ...
)
```

Tidak ada constraint yang memastikan:

```text
1 Contact = 1 Structured Name
```

Secara desain Google/Android contact, satu contact biasanya memiliki satu primary structured name.

Saya sarankan:

```sql
UNIQUE(contact_id)
```

Menjadi:

```sql
CREATE TABLE IF NOT EXISTS contact_names (
    id TEXT PRIMARY KEY,

    contact_id TEXT NOT NULL UNIQUE,

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
```

Tabel Anda saat ini memang memiliki relasi `contact_id`, tetapi belum membatasi satu nama terstruktur per contact.

---

# 2. `is_primary` belum dijamin hanya satu

Contoh:

```text
Contact A

Phone 1 → primary = 1
Phone 2 → primary = 1
Phone 3 → primary = 1
```

SQLite tidak akan mencegah kondisi tersebut.

Sebaiknya gunakan partial unique index:

```sql
CREATE UNIQUE INDEX IF NOT EXISTS idx_one_primary_phone
ON contact_phones(contact_id)
WHERE is_primary = 1;
```

Untuk email:

```sql
CREATE UNIQUE INDEX IF NOT EXISTS idx_one_primary_email
ON contact_emails(contact_id)
WHERE is_primary = 1;
```

Saat ini tabel phone dan email sudah memiliki `is_primary`, tetapi constraint tersebut belum ada.

---

# 3. Tambahkan `source_id` untuk restore

Ini menurut saya sangat penting.

Saat ini:

```text
contacts
├── id
├── snapshot_id
├── source
└── source_account
```

Tetapi belum ada ID asli dari sumber.

Tambahkan:

```sql
source_id TEXT
```

Contoh:

```text
Google People API
resourceName = people/c123456

Android Contacts Provider
contact_id = 123

SIM
record_id = 8
```

Schema:

```sql
CREATE TABLE IF NOT EXISTS contacts (
    id TEXT PRIMARY KEY,

    snapshot_id TEXT NOT NULL,

    source_id TEXT,

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
```

Saya juga menyarankan index:

```sql
CREATE INDEX IF NOT EXISTS idx_contacts_source
ON contacts(source, source_id);
```

---

# 4. Phone number perlu `raw_value`

Saat ini:

```text
value
normalized_value
```

Bagus, tetapi untuk backup sebaiknya dibedakan:

```text
raw_value
display_value
normalized_value
```

Contoh:

```text
raw_value
0812-1234-5678

display_value
0812-1234-5678

normalized_value
+6281212345678
```

Karena backup harus sebisa mungkin menyimpan data asli.

Rekomendasi:

```sql
CREATE TABLE contact_phones (
    id TEXT PRIMARY KEY,

    contact_id TEXT NOT NULL,

    raw_value TEXT NOT NULL,

    normalized_value TEXT,

    type TEXT,

    label TEXT,

    is_primary INTEGER NOT NULL DEFAULT 0,

    FOREIGN KEY(contact_id)
        REFERENCES contacts(id)
        ON DELETE CASCADE
);
```

Tabel phone Anda saat ini sudah memiliki index untuk pencarian nomor yang dinormalisasi.

---

# 5. Contact tidak memiliki checksum / fingerprint

Untuk sistem backup incremental, ini sangat penting.

Tambahkan:

```sql
content_hash TEXT
```

Contoh:

```text
Contact:

Damar Kuncoro
+6281212345678
damar@example.com
PT Example
```

Dinormalisasi:

```text
damar kuncoro
+6281212345678
damar@example.com
pt example
```

Kemudian:

```text
SHA256
↓
A83F1D...
```

Schema:

```sql
ALTER TABLE contacts
ADD COLUMN content_hash TEXT;
```

Index:

```sql
CREATE INDEX IF NOT EXISTS idx_contacts_content_hash
ON contacts(content_hash);
```

Ini memungkinkan:

```text
Snapshot 001
Contact Hash ABC

Snapshot 002
Contact Hash ABC

↓
Tidak berubah
Tidak perlu menyimpan ulang data fisik
```

---

# 6. Masalah arsitektur snapshot

Saat ini:

```text
Snapshot
   │
   └── Contacts
```

Artinya setiap snapshot menyimpan seluruh contact.

Untuk Phase awal, ini **tidak masalah**.

Tetapi nanti ketika:

```text
100.000 contacts
500 snapshots
```

akan terjadi banyak duplikasi.

Saya menyarankan arsitektur masa depan:

```text
contact_objects
       │
       │ immutable
       ▼
snapshot_contacts
       │
       ▼
snapshots
```

Seperti file backup:

```text
files
       │
       ▼
snapshot_files
       │
       ▼
snapshots
```

Desain:

```text
contact_objects
│
├── id
├── content_hash
├── contact_data
└── created_at

snapshot_contacts
│
├── snapshot_id
└── contact_object_id
```

Tetapi untuk sekarang, relational schema Anda masih sangat baik untuk Phase 01.

---

# 7. Contact Labels perlu `source_account`

Saat ini label:

```text
snapshot_id
name
source_id
```

Saya sarankan:

```sql
source TEXT,
source_account TEXT
```

Contoh:

```text
source = google
source_account = user@gmail.com

Label:
Work
Family
Friends
```

Ini penting jika satu device memiliki:

```text
Google Account A
Google Account B
Exchange
Local Phone
```

---

# 8. Contact photo perlu `hash`

Saat ini:

```text
contact_photos
├── contact_id
├── file_id
├── mime_type
└── is_primary
```

Relasi ke file sudah bagus.

Tetapi untuk deduplication:

```sql
photo_hash TEXT
```

akan membantu.

Contoh:

```text
Photo A
SHA256 = ABC123

Contact 1 ─┐
Contact 2 ─┼──> File Object ABC123
Contact 3 ─┘
```

---

# 9. Tambahkan metadata Android/Google yang fleksibel

Google Contacts dan Android Contacts memiliki banyak field yang mungkin bertambah.

Daripada setiap perubahan schema membutuhkan migration, tambahkan:

```sql
metadata_json TEXT
```

Pada `contacts`:

```sql
metadata_json TEXT
```

Contoh:

```json
{
  "android_contact_id": "123",
  "lookup_key": "abc:def:ghi",
  "google_resource_name": "people/c12345"
}
```

Ini membuat engine Anda lebih tahan terhadap perubahan adapter.

---

# Arsitektur yang saya rekomendasikan

```text
                         DEVICE
                           │
                           ▼
                       SNAPSHOT
                           │
                           │
                           ▼
                        CONTACTS
                           │
       ┌───────────────────┼───────────────────┐
       │                   │                   │
       ▼                   ▼                   ▼
   STRUCTURED NAME       PHONES              EMAILS
       │                   │                   │
       └───────────────────┼───────────────────┘
                           │
          ┌────────────────┼─────────────────┐
          │                │                 │
          ▼                ▼                 ▼
       ADDRESS       ORGANIZATION           URL
          │
          ▼
        EVENTS

CONTACTS
   │
   ├──────────────► CONTACT PHOTOS
   │                      │
   │                      ▼
   │                    FILES
   │
   └──────────────► CONTACT LABEL MEMBERS
                           │
                           ▼
                    CONTACT LABELS
```

## Status schema Anda

| Bagian                | Status                     |
| --------------------- | -------------------------- |
| Device                | 🟢 Baik                    |
| Files                 | 🟢 Baik                    |
| Snapshot              | 🟢 Baik                    |
| Apps                  | 🟢 Baik                    |
| Deduplication         | 🟡 Perlu pengembangan      |
| Contacts              | 🟢 Sangat baik             |
| Google compatibility  | 🟢 Baik                    |
| Android compatibility | 🟢 Baik                    |
| Incremental backup    | 🟡 Belum optimal           |
| Restore engine        | 🟡 Perlu desain berikutnya |

**Langkah berikutnya yang paling tepat adalah membuat Rust domain model (`Contact`, `ContactPhone`, `ContactEmail`, dan seterusnya), lalu Repository Pattern untuk `rusqlite`, sebelum masuk ke Android Contact Adapter.**
