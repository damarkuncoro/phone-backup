Jika Anda ingin desain **Contact Backup yang kompatibel dengan struktur Google Contacts**, sebaiknya jangan menyimpan kontak dalam satu tabel dengan kolom JSON seperti:

```text
phones
emails
addresses
organizations
```

Karena satu kontak di Google Contacts bisa memiliki **banyak nomor telepon, email, alamat, organisasi, nama, event, URL, dan custom field**.

![Image](https://images.openai.com/static-rsc-4/F9z6OGKt3IKwd661k7y6Dy5qnTGoYDz2mYUkLk3aIa5qFupY8nXsCQtxRF6ypG58SbJPcu8BOMqEukSPBFXeMs3SuoRCNJGik1dzgDDpscLgtbWFmBsS7q0gBe35kdZx2Vvmwx9vQxg4s2nsNO_-gFsTx6OnmzNqjqqO3T0vtVbcy62vXJ2R4F4xO9Ak9Nj6?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/gKKtZOwOspbkrLErGTz5PuFYcHGscNc8ezXl3RZyN16ikiR4-UBVFqIEl9_7NI1K3vq5GVUose6KujoGLh_ewT7jN7758JZYEWEWBdyt5Ae_FV8maCN_LnMPOCfnEkyRy8WfJZxG6Fxpaalc0vB544jUKtbao4464qpUQ60soO3ucAJIH-hAwgVA8QbCcCYG?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/iArq1sgvgCkXJRrW1Mdd-Sc0AKfnY0-V97plhB61YtdII_jvteyES-2MPoHN1qIX_wPclqexmLovJg14DhMCKKx_e2yddI7Sx1J_w6dz30KeCJaMA_iqmcaNMTBXTsWpiRlM5rvj7M3njPrEccpvCGO9tgprxEvkIKGYG3_sCaUek0FpBonGuGOucfwh2TYi?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/ZeVzfm6_1ydS96p8NJsco6EV5Irj4dEAMh_2mQzKfobLkIPupxw0GItdqMG36KQySfxer8uaGRMGvebKEaDPvmOwRNMz5B7eKftanz7WcfevJkeNNBrV7_0L0hzgMdma8rwQu4r1_qhhiQu8TZ1BnCz4B0zpj2UHIE4SO_XDfkguK3u493flRCK9VJXHfNyf?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/4UQbSvob3HUcF0KbEs5KZhC_Yv2freVNVXo3MDcpzA_4_TvrUnPkKulrsiJhlBuEVf_ouWJbLWrJxlzATQQYw7D3-Il5y2Ew_AeHBjuRz1R2zTFklSVxTr7D31_B25bqyAXbyQYjtEr8gd48mPTrUd9NhASNVNZGAz6KJKJn0CB2VoS74IzhIlTm-rUyle09?purpose=fullsize)

## 1. Konsep data Google Contacts

Secara konseptual, satu contact memiliki struktur seperti:

```text
CONTACT
│
├── Names
│   ├── Display Name
│   ├── Given Name
│   ├── Family Name
│   ├── Middle Name
│   ├── Prefix
│   └── Suffix
│
├── Phone Numbers
│   ├── Mobile
│   ├── Home
│   ├── Work
│   └── Other
│
├── Emails
│   ├── Personal
│   ├── Work
│   └── Other
│
├── Addresses
│
├── Organizations
│
├── URLs
│
├── Birthdays
│
├── Photos
│
├── Notes
│
└── Groups / Labels
```

Contoh:

```text
Damar Kuncoro
│
├── 📱 Mobile
│   ├── +62 812xxxxxxx
│   └── +62 813xxxxxxx
│
├── 📧 Email
│   ├── damarkuncoro@example.com
│   └── kerja@example.com
│
├── 🏢 Organization
│   └── PT Cakramedia Indocyber
│
└── 🏷 Label
    ├── Family
    └── Work
```

---

# 2. Database design yang direkomendasikan

Saya menyarankan struktur relational.

```text
contacts
    │
    ├── contact_names
    │
    ├── contact_phones
    │
    ├── contact_emails
    │
    ├── contact_addresses
    │
    ├── contact_organizations
    │
    ├── contact_urls
    │
    ├── contact_events
    │
    ├── contact_photos
    │
    └── contact_labels
             │
             └── labels
```

---

# 3. Tabel utama `contacts`

```sql
CREATE TABLE contacts (
    id TEXT PRIMARY KEY,

    snapshot_id TEXT NOT NULL,

    source_id TEXT,

    display_name TEXT NOT NULL,

    notes TEXT,

    created_at TEXT NOT NULL,

    updated_at TEXT,

    FOREIGN KEY(snapshot_id)
        REFERENCES snapshots(id)
        ON DELETE CASCADE
);
```

Contoh:

| id          | source_id         | display_name  |
| ----------- | ----------------- | ------------- |
| contact_001 | google_people_abc | Damar Kuncoro |

`source_id` penting jika nanti Anda melakukan:

```text
Google Contacts
       ↓
Backup Tool
       ↓
Restore
       ↓
Google Contacts
```

---

# 4. Nama Contact

Google Contact bisa memiliki struktur nama cukup kompleks.

```sql
CREATE TABLE contact_names (
    id TEXT PRIMARY KEY,

    contact_id TEXT NOT NULL,

    display_name TEXT,

    given_name TEXT,

    middle_name TEXT,

    family_name TEXT,

    prefix TEXT,

    suffix TEXT,

    phonetic_given_name TEXT,

    phonetic_middle_name TEXT,

    phonetic_family_name TEXT,

    FOREIGN KEY(contact_id)
        REFERENCES contacts(id)
        ON DELETE CASCADE
);
```

Contoh:

```text
Display Name
Damar Kuncoro

Given Name
Damar

Family Name
Kuncoro
```

---

# 5. Phone Numbers

Jangan gunakan:

```text
phones TEXT
```

Lebih baik:

```sql
CREATE TABLE contact_phones (
    id TEXT PRIMARY KEY,

    contact_id TEXT NOT NULL,

    value TEXT NOT NULL,

    normalized_value TEXT,

    type TEXT NOT NULL,

    label TEXT,

    is_primary INTEGER DEFAULT 0,

    FOREIGN KEY(contact_id)
        REFERENCES contacts(id)
        ON DELETE CASCADE
);
```

Contoh:

```text
contact_001

Mobile
+62 812 1234 5678

Work
+62 21 1234 5678
```

Data:

| contact_id  | value          | type   |
| ----------- | -------------- | ------ |
| contact_001 | +6281212345678 | mobile |
| contact_001 | +622112345678  | work   |

---

# 6. Email

```sql
CREATE TABLE contact_emails (
    id TEXT PRIMARY KEY,

    contact_id TEXT NOT NULL,

    value TEXT NOT NULL,

    type TEXT NOT NULL,

    label TEXT,

    is_primary INTEGER DEFAULT 0,

    FOREIGN KEY(contact_id)
        REFERENCES contacts(id)
        ON DELETE CASCADE
);
```

Contoh:

```text
Personal
damar@gmail.com

Work
damar@company.com
```

---

# 7. Address

Alamat perlu dipisahkan karena strukturnya kompleks.

```sql
CREATE TABLE contact_addresses (
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
```

---

# 8. Organization

Satu contact dapat memiliki lebih dari satu organisasi.

```sql
CREATE TABLE contact_organizations (
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
```

Contoh:

```text
Company
PT Cakramedia Indocyber

Department
Network Engineering

Title
Network Engineer
```

---

# 9. URL

```sql
CREATE TABLE contact_urls (
    id TEXT PRIMARY KEY,

    contact_id TEXT NOT NULL,

    url TEXT NOT NULL,

    type TEXT,

    label TEXT,

    FOREIGN KEY(contact_id)
        REFERENCES contacts(id)
        ON DELETE CASCADE
);
```

Contoh:

```text
Website
https://example.com

Profile
https://linkedin.com/...
```

---

# 10. Events

Google Contacts juga mendukung event.

Contohnya:

```text
Birthday
Anniversary
Custom Date
```

Schema:

```sql
CREATE TABLE contact_events (
    id TEXT PRIMARY KEY,

    contact_id TEXT NOT NULL,

    event_type TEXT NOT NULL,

    event_date TEXT NOT NULL,

    label TEXT,

    FOREIGN KEY(contact_id)
        REFERENCES contacts(id)
        ON DELETE CASCADE
);
```

---

# 11. Contact Photo

Untuk backup tool HP, jangan menyimpan foto langsung di SQLite.

Lebih baik:

```text
SQLite
   │
   │ metadata
   ▼
photo_path / object_id
   │
   ▼
Backup Storage
```

Schema:

```sql
CREATE TABLE contact_photos (
    id TEXT PRIMARY KEY,

    contact_id TEXT NOT NULL,

    object_id TEXT,

    mime_type TEXT,

    width INTEGER,

    height INTEGER,

    is_primary INTEGER DEFAULT 1,

    FOREIGN KEY(contact_id)
        REFERENCES contacts(id)
        ON DELETE CASCADE
);
```

---

# 12. Labels / Groups

Ini sangat penting untuk kompatibilitas dengan Google Contacts.

```text
Contacts
    │
    └── Contact Labels
              │
              ├── Family
              ├── Work
              └── Friends
```

Schema:

```sql
CREATE TABLE contact_labels (
    id TEXT PRIMARY KEY,

    snapshot_id TEXT NOT NULL,

    name TEXT NOT NULL,

    source_id TEXT,

    FOREIGN KEY(snapshot_id)
        REFERENCES snapshots(id)
        ON DELETE CASCADE
);
```

Relasi many-to-many:

```sql
CREATE TABLE contact_label_members (
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
```

---

# 13. Full relational diagram

```text
                         ┌───────────────┐
                         │   SNAPSHOT    │
                         └───────┬───────┘
                                 │
                                 ▼
                         ┌───────────────┐
                         │   CONTACTS    │
                         └───────┬───────┘
                                 │
          ┌──────────────┬───────┼──────────┬─────────────┐
          │              │       │          │             │
          ▼              ▼       ▼          ▼             ▼

       NAMES          PHONES   EMAILS   ADDRESSES   ORGANIZATIONS

          │              │       │          │             │

          └──────────────┴───────┴──────────┴─────────────┘

                                 │

                 ┌───────────────┼────────────────┐
                 │               │                │
                 ▼               ▼                ▼

                URLS           EVENTS           PHOTOS

                                 │

                                 ▼

                         LABEL MEMBERS
                                 │
                                 ▼
                              LABELS
```

---

# 14. Untuk backup: tambahkan metadata source

Saya sangat menyarankan setiap contact memiliki informasi asal.

```sql
source TEXT NOT NULL
```

Contoh:

```text
google
android
sim
phone
whatsapp
exchange
icloud
local
```

Dan:

```sql
source_account TEXT
```

Contoh:

```text
damar@gmail.com
user@company.com
device-local
SIM1
```

Maka:

```sql
ALTER TABLE contacts
ADD COLUMN source TEXT NOT NULL DEFAULT 'unknown';

ALTER TABLE contacts
ADD COLUMN source_account TEXT;
```

Ini sangat berguna ketika restore.

---

# 15. Desain terbaik untuk project Anda

Karena tool yang sedang Anda bangun adalah **backup HP dari nol**, saya akan menggunakan tiga layer:

```text
ANDROID / GOOGLE CONTACTS
           │
           │ Import Adapter
           ▼
┌──────────────────────────────┐
│      CONTACT NORMALIZER      │
│                              │
│ Google People API            │
│ Android Contacts Provider    │
│ vCard Import                 │
│ SIM Contact Import           │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│     INTERNAL CONTACT MODEL   │
│                              │
│ Contact                      │
│ ├── Names                    │
│ ├── Phones                   │
│ ├── Emails                   │
│ ├── Addresses                │
│ ├── Organizations            │
│ ├── Events                   │
│ ├── URLs                     │
│ ├── Photos                   │
│ └── Labels                   │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│       SQLITE BACKUP DB       │
└──────────────────────────────┘
               │
               ▼
        RESTORE ADAPTER
               │
      ┌────────┼─────────┐
      ▼        ▼         ▼
   Android   Google     vCard
```

### Rekomendasi utama

Untuk project Anda, **jangan desain database mengikuti persis UI Google Contacts**. Lebih baik buat **Internal Contact Model** yang cukup kaya untuk menampung:

* Google Contacts
* Android Contacts Provider
* vCard
* SIM contacts
* iCloud contacts

Kemudian buat adapter:

```text
GoogleContactAdapter
AndroidContactAdapter
VCardAdapter
SimContactAdapter
```

Dengan desain ini, tool backup Anda tidak hanya menjadi **Google Contacts backup**, tetapi menjadi **Universal Contact Backup Engine** yang dapat melakukan:

```text
Android → Backup → SQLite
Google → Backup → SQLite
vCard → Backup → SQLite

SQLite → Android
SQLite → Google-compatible export
SQLite → vCard
```

Struktur ini sangat cocok untuk dikembangkan sebagai bagian dari **Phase Backup Contacts**, lalu dilanjutkan ke SMS, Call Log, Media, Documents, dan App Inventory.
