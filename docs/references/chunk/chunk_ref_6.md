v3.0 ini sudah jauh lebih matang. Saya setuju menjadikannya **baseline arsitektur coding**, tetapi saya belum akan menyebutnya 100% final tanpa beberapa koreksi kecil yang cukup penting.

## Verdict

**Secara keseluruhan: 9/10 — siap masuk tahap implementasi.**

Yang paling bagus dari v3.0:

* dedup dilakukan sebelum kompresi/enkripsi;
* plaintext hash dipisahkan dari object hash;
* snapshot menjadi bagian dari model;
* `ref_count` diposisikan sebagai cache;
* ada crash-safe GC;
* pipeline streaming;
* strategi chunking dibedakan berdasarkan tipe data.

Namun ada **6 hal yang saya ubah sebelum kode ditulis**.

---

# 1. Jangan gunakan `content_hash` saja sebagai satu-satunya identitas chunk

Saat ini:

```sql
content_hash BLOB UNIQUE
```

Ini bagus untuk deduplication, tetapi ada konsekuensi terhadap encryption version.

Misalnya:

```text
Chunk X
BLAKE3 = ABC123
```

Kemudian:

```text
Encryption v1
→ Object A

Encryption v2
→ Object B
```

Content-nya sama, tetapi representasi storage berbeda.

Karena itu lebih aman memisahkan:

```text
Content Identity
        │
        ▼
content_hash

Physical Object
        │
        ├── object_id
        ├── object_hash
        ├── compression
        ├── encryption_version
        └── storage_key
```

Saya akan mengubah model menjadi:

```text
content
   │
   ├── content_hash
   └── size
        │
        ▼
physical_objects
   │
   ├── object_hash
   ├── compression
   ├── encryption
   └── storage_key
```

Dengan demikian **logical content** tidak terikat dengan satu format storage.

---

# 2. `Storage Key = UUID` saya lebih suka untuk encrypted storage

Anda menulis:

> Storage Key: UUID atau hash unik yang tidak membocorkan isi plaintext.

Saya memilih:

```text
UUIDv7 / random object ID
```

daripada plaintext-derived hash.

Contoh:

```text
objects/
└── 0199c8b2-....
```

Bukan:

```text
objects/
└── a8f91c....   ← derived dari content
```

Alasannya:

* tidak memberikan identifier yang mudah dikorelasikan;
* memisahkan content identity dari physical storage;
* memudahkan migrasi;
* dapat mengganti encryption/compression tanpa mengubah logical content ID.

---

# 3. Ada satu masalah penting pada Convergent Encryption

Desain:

```text
BLAKE3(plaintext)
       │
       ▼
      HKDF
       │
       ▼
  Chunk Encryption Key
```

lebih baik daripada menggunakan hash langsung sebagai key.

Tetapi **HKDF tidak menghilangkan equality leakage**.

Jika:

```text
Chunk A == Chunk B
```

maka:

```text
content_hash A == content_hash B
```

dan jika encryption deterministic:

```text
ciphertext A == ciphertext B
```

atau setidaknya storage identity dapat dikorelasikan.

Jadi desain ini harus dianggap sebagai:

> **deduplication-friendly encryption dengan equality leakage**

bukan encryption yang menyembunyikan apakah dua plaintext sama.

Untuk backup pribadi lokal, trade-off tersebut bisa diterima.

Untuk cloud/multi-user service, threat model harus jauh lebih ketat.

---

# 4. Saya akan menambahkan `backup_manifests`

Saat ini:

```text
snapshot
   │
   ▼
snapshot_files
   │
   ▼
files
   │
   ▼
file_chunks
   │
   ▼
chunks
```

Ini sudah bagus.

Tetapi untuk restore yang benar-benar **point-in-time**, saya ingin ada manifest immutable.

```text
Snapshot
   │
   ▼
Manifest
   │
   ├── File metadata
   ├── Chunk sequence
   ├── Encryption version
   ├── Compression
   └── Format version
```

Contoh:

```sql
CREATE TABLE snapshot_manifests (
    snapshot_id BLOB PRIMARY KEY,
    manifest_hash BLOB NOT NULL,
    format_version INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (snapshot_id)
        REFERENCES snapshots(id)
);
```

Manifest bisa menjadi **cryptographic root** untuk sebuah backup.

---

# 5. GC harus memakai Snapshot Commit Protocol

Ini sangat penting.

Jangan:

```text
backup
  ↓
write chunks
  ↓
snapshot COMPLETED
```

secara sederhana.

Gunakan:

```text
             BEGIN
               │
               ▼
          SNAPSHOT PENDING
               │
               ▼
         Write metadata
               │
               ▼
         Write objects
               │
               ▼
        Verify objects
               │
               ▼
        Commit manifest
               │
               ▼
       SNAPSHOT COMPLETED
```

Jika crash:

```text
PENDING
  │
  ├── recovery
  │
  ├── complete
  │
  └── rollback
```

Baru object yang tidak digunakan bisa menjadi:

```text
CANDIDATE
```

---

# 6. `ref_count_cache` jangan di-update sebagai bagian kritis setiap reference

Saya akan memperlakukan:

```sql
ref_count_cache
```

sebagai **performance optimization**.

Source of Truth:

```text
snapshots
    ↓
snapshot_files
    ↓
file_chunks
    ↓
chunks
```

GC dapat menghitung:

```sql
SELECT chunk_id, COUNT(*)
FROM file_chunks
GROUP BY chunk_id;
```

kemudian membandingkan dengan cache.

Jika berbeda:

```text
cache = 17
actual = 18
```

maka:

```text
repair cache
```

Bukan langsung delete.

---

# 7. Saya akan sedikit mengubah schema final

Saya lebih menyukai desain ini:

```sql
CREATE TABLE snapshots (
    id BLOB PRIMARY KEY,
    device_id BLOB NOT NULL,
    created_at INTEGER NOT NULL,
    status TEXT NOT NULL,
    format_version INTEGER NOT NULL
);
```

```sql
CREATE TABLE files (
    id BLOB PRIMARY KEY,
    path TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    modified_at INTEGER,
    content_hash BLOB
);
```

```sql
CREATE TABLE snapshot_files (
    snapshot_id BLOB NOT NULL,
    file_id BLOB NOT NULL,

    PRIMARY KEY (snapshot_id, file_id),

    FOREIGN KEY (snapshot_id)
        REFERENCES snapshots(id),

    FOREIGN KEY (file_id)
        REFERENCES files(id)
);
```

Kemudian:

```sql
CREATE TABLE chunks (
    id BLOB PRIMARY KEY,

    content_hash BLOB NOT NULL UNIQUE,
    plaintext_size INTEGER NOT NULL,

    created_at INTEGER NOT NULL
);
```

Dan physical object:

```sql
CREATE TABLE chunk_objects (
    id BLOB PRIMARY KEY,

    chunk_id BLOB NOT NULL,

    object_hash BLOB NOT NULL UNIQUE,
    storage_key TEXT NOT NULL,

    stored_size INTEGER NOT NULL,

    compression_alg TEXT NOT NULL,
    encryption_alg TEXT NOT NULL,
    encryption_version INTEGER NOT NULL,

    FOREIGN KEY (chunk_id)
        REFERENCES chunks(id)
);
```

Kemudian:

```sql
CREATE TABLE file_chunks (
    file_id BLOB NOT NULL,
    chunk_id BLOB NOT NULL,

    position INTEGER NOT NULL,
    offset_bytes INTEGER NOT NULL,

    PRIMARY KEY (file_id, position),

    FOREIGN KEY (file_id)
        REFERENCES files(id),

    FOREIGN KEY (chunk_id)
        REFERENCES chunks(id)
);
```

Dengan model ini:

```text
                 LOGICAL
                    │
                    ▼
                  CHUNK
                    │
             content_hash
                    │
                    ▼
              CHUNK_OBJECT
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
    compression  encryption  storage
```

Ini menurut saya lebih fleksibel daripada memasukkan seluruh physical storage metadata langsung ke `chunks`.

---

# 8. Satu koreksi pada taxonomy

Saya akan sedikit mengubah:

> Media JPG/HEIC → File-Level

Tidak selalu demikian.

Untuk backup biasa:

```text
JPG/HEIC
→ File-level dedup
```

**sangat masuk akal**.

Tetapi jangan membuat engine tidak bisa melakukan CDC terhadapnya.

Karena pada beberapa workflow:

```text
Photo editing
EXIF changes
Metadata rewriting
Container modification
```

dapat membuat file berubah meskipun payload gambar sebagian besar sama.

Jadi sebaiknya:

```rust
pub enum ChunkingStrategy {
    FileLevel,
    FixedSize(FixedConfig),
    FastCdc(FastCdcConfig),
    PackedSmallFiles(PackingConfig),
}
```

dan:

```rust
pub trait ChunkingPolicy {
    fn choose(&self, file: &FileDescriptor)
        -> ChunkingStrategy;
}
```

Policy yang memilih, bukan file type yang mengunci engine.

---

# 9. Final Pipeline v3.1

Jika saya merapikan v3.0 Anda, saya akan menjadikan pipeline resmi:

```text
┌──────────────────────┐
│   DEVICE / ADB       │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Scanner              │
│ File Metadata        │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ File Classifier      │
└──────────┬───────────┘
           │
           ├──── Small ────► Small File Packer
           │
           └──── Large ────► Streaming Reader
                                  │
                                  ▼
                         ┌─────────────────┐
                         │ Chunking Policy │
                         └────────┬────────┘
                                  │
                     ┌────────────┼────────────┐
                     ▼            ▼            ▼
                   File        Fixed        FastCDC
                     │            │            │
                     └────────────┼────────────┘
                                  ▼
                             BLAKE3
                                  │
                                  ▼
                           Dedup Lookup
                           /           \
                        HIT             MISS
                         │                │
                         ▼                ▼
                      REUSE             Zstd
                      REF                │
                                         ▼
                                      HKDF
                                         │
                                         ▼
                              XChaCha20-Poly1305
                                         │
                                         ▼
                                  Object Hash
                                         │
                                         ▼
                                Object Storage
                                         │
                                         ▼
                                Snapshot Commit
                                         │
                                         ▼
                                    MANIFEST
```

---

# 10. Tambahkan satu komponen: Recovery Manager

Ini belum terlihat jelas dalam v3.0.

Production backup harus mengantisipasi:

```text
Power loss
USB disconnect
ADB disconnect
Disk full
Process killed
SQLite crash
Network timeout
Object write failure
```

Maka:

```text
RecoveryManager
```

menjadi komponen penting.

```text
Backup
  │
  ▼
Journal / State
  │
  ▼
Crash
  │
  ▼
Recovery Manager
  │
  ├── Resume
  ├── Rollback
  ├── Repair
  └── Mark Failed
```

---

# 11. Maka arsitektur finalnya

```text
phone-backup/
│
├── core/
│   ├── domain/
│   │   ├── device/
│   │   ├── file/
│   │   ├── chunk/
│   │   ├── snapshot/
│   │   └── manifest/
│   │
│   ├── application/
│   │   ├── backup/
│   │   ├── restore/
│   │   ├── recovery/
│   │   ├── gc/
│   │   └── scrubbing/
│   │
│   └── ports/
│       ├── scanner.rs
│       ├── chunker.rs
│       ├── hasher.rs
│       ├── compressor.rs
│       ├── encryptor.rs
│       ├── dedup.rs
│       ├── storage.rs
│       └── repository.rs
│
├── adapters/
│   ├── adb/
│   ├── sqlite/
│   ├── filesystem/
│   └── object_storage/
│
└── infrastructure/
    ├── chunking/
    │   ├── fixed.rs
    │   ├── fastcdc.rs
    │   └── file_level.rs
    │
    ├── crypto/
    │   ├── hkdf.rs
    │   └── xchacha20poly1305.rs
    │
    ├── compression/
    │   └── zstd.rs
    │
    └── hashing/
        └── blake3.rs
```

## Jadi saya akan menetapkan status dokumen Anda:

**v3.0 → APPROVED WITH CHANGES**

Dan setelah perubahan di atas, kita bisa membuat:

> **Phone Backup Engine Technical Specification v4.0**

yang sudah cukup stabil untuk diterjemahkan langsung menjadi **Rust workspace + SQLite migrations + traits + domain entities + pipeline implementation**.

Urutan coding yang paling aman adalah **Snapshot/Repository → Chunk Core → Fixed Chunker → BLAKE3/Dedup → Object Storage → Transaction/Recovery → Zstd → Crypto → FastCDC → GC → Scrubbing**. Dengan urutan itu, setiap fase bisa dites secara independen sebelum kompleksitas berikutnya masuk.
