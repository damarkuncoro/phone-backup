**Disetujui.** v4.0 sudah cukup solid untuk dijadikan **baseline implementasi Phone Backup Engine**.

Saya hanya akan menetapkan beberapa **invariant teknis** agar ketika coding dimulai, implementasi tidak menyimpang dari spesifikasi.

## 1. Kontrak utama yang harus dikunci

```text
Plaintext
   │
   ├── BLAKE3 ───────────► content_hash
   │
   ▼
Dedup Lookup
   │
   ├── HIT ──────────────► reuse chunk_id
   │
   └── MISS
         │
         ▼
       Zstd
         │
         ▼
       HKDF
         │
         ▼
 XChaCha20-Poly1305
         │
         ▼
   ciphertext
         │
         ├── BLAKE3 ────► object_hash
         │
         ▼
    UUIDv7 Object
```

**Aturan:** `content_hash` adalah identitas logical content; `object_hash` adalah identitas/integritas physical representation.

---

# 2. Invariant database

Kita harus menjaga:

```text
content_hash UNIQUE
object_hash  UNIQUE
storage_key  UNIQUE
```

Dan:

```text
chunk_id
   │
   └── 1..N
        │
        ▼
chunk_objects
```

Ini memungkinkan satu logical chunk memiliki beberapa physical representation di masa depan.

Misalnya:

```text
Chunk A
│
├── Object v1 → Zstd + XChaCha20
└── Object v2 → New compression + New encryption
```

Ini sangat bagus untuk migrasi storage.

---

# 3. Tambahkan constraint yang belum ada

Saya menyarankan schema final sedikit diperketat:

```sql
CREATE TABLE chunks (
    id BLOB PRIMARY KEY,
    content_hash BLOB NOT NULL UNIQUE,
    plaintext_size INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE chunk_objects (
    id BLOB PRIMARY KEY,

    chunk_id BLOB NOT NULL,

    object_hash BLOB NOT NULL UNIQUE,
    storage_key TEXT NOT NULL UNIQUE,

    stored_size INTEGER NOT NULL,

    compression_alg TEXT NOT NULL,
    encryption_version INTEGER NOT NULL,

    FOREIGN KEY (chunk_id)
        REFERENCES chunks(id)
);
```

Tambahkan index:

```sql
CREATE INDEX idx_chunk_objects_chunk_id
ON chunk_objects(chunk_id);
```

---

# 4. Satu bagian yang masih perlu ditambahkan: `file_chunks`

v4.0 menampilkan schema `chunks` dan `chunk_objects`, tetapi relasi urutan chunk dalam file harus tetap dikunci.

```sql
CREATE TABLE file_chunks (
    file_id BLOB NOT NULL,
    chunk_id BLOB NOT NULL,

    position INTEGER NOT NULL,
    offset_bytes INTEGER NOT NULL,
    length_bytes INTEGER NOT NULL,

    PRIMARY KEY (file_id, position),

    FOREIGN KEY (file_id)
        REFERENCES files(id),

    FOREIGN KEY (chunk_id)
        REFERENCES chunks(id)
);
```

Ini sangat penting untuk restore:

```text
file
 │
 ├── position 0 → chunk A
 ├── position 1 → chunk B
 ├── position 2 → chunk C
 └── position 3 → chunk D
```

Tanpa `position`, kita tidak bisa menjamin urutan reconstruction.

---

# 5. Snapshot harus immutable setelah COMPLETED

State machine:

```text
                    ┌────────────┐
                    │   BEGIN    │
                    └─────┬──────┘
                          ▼
                    ┌────────────┐
                    │  PENDING   │
                    └─────┬──────┘
                          │
                 ┌────────┴────────┐
                 ▼                 ▼
             RECOVERY            VERIFY
                 │                 │
                 │                 ▼
                 │              COMMIT
                 │                 │
                 │                 ▼
                 │            COMPLETED
                 │
                 ▼
              FAILED
```

Setelah:

```text
COMPLETED
```

snapshot **tidak boleh dimodifikasi**.

Jika user menghapus backup:

```text
COMPLETED
      │
      ▼
DELETE REQUEST
      │
      ▼
Tombstone / Deleted
      │
      ▼
GC
```

Ini membuat snapshot jauh lebih mudah diverifikasi.

---

# 6. Manifest sebagai cryptographic root

Ini saya anggap **wajib** untuk implementasi final.

Contohnya:

```text
Snapshot
   │
   ▼
Manifest
   │
   ├── File A
   │     ├── Chunk A
   │     └── Chunk B
   │
   ├── File B
   │     └── Chunk C
   │
   └── File C
         └── Chunk D
```

Kemudian:

```text
Manifest
   │
   ▼
BLAKE3
   │
   ▼
manifest_hash
```

Sehingga kita bisa melakukan:

```text
Snapshot Integrity Check
```

dengan satu root identity.

---

# 7. Encryption harus memiliki versioning

Jangan hanya:

```sql
encryption_version INTEGER
```

tetapi secara domain:

```rust
pub enum EncryptionVersion {
    V1,
}
```

Nanti:

```rust
pub enum EncryptionVersion {
    V1,
    V2,
}
```

Engine restore harus membaca:

```text
encryption_version
       │
       ▼
EncryptionRegistry
       │
       ├── V1 → decrypt_v1()
       └── V2 → decrypt_v2()
```

Jangan pernah mengubah algoritma V1 secara diam-diam.

---

# 8. Chunking juga harus versioned

Sama seperti encryption.

Manifest sebaiknya mengetahui:

```text
chunking_strategy
chunking_version
```

Misalnya:

```text
chunking_version = 1
strategy = fixed-4MiB
```

Backup berikutnya:

```text
chunking_version = 2
strategy = fastcdc
```

Backup lama tetap bisa direstore.

---

# 9. Recovery invariant

Recovery tidak boleh mengandalkan:

```text
ref_count_cache
```

sebagai kebenaran.

Source of Truth:

```text
Snapshot
   ↓
Snapshot Files
   ↓
Files
   ↓
File Chunks
   ↓
Chunks
```

Sedangkan:

```text
ref_count_cache
```

hanya:

```text
CACHE
```

Jika crash:

```text
Actual References
       │
       ▼
Recalculate
       │
       ▼
Repair Cache
```

---

# 10. Object storage harus atomic

Jangan langsung:

```text
write object
```

Gunakan pola:

```text
TEMP OBJECT
    │
    ▼
fsync / verify
    │
    ▼
ATOMIC RENAME
    │
    ▼
FINAL OBJECT
```

Contoh:

```text
objects/
├── .tmp/
│   └── upload-123
│
└── 01/
    └── 0199c8b2-...
```

Jika aplikasi crash di tengah:

```text
.tmp/upload-123
```

tidak dianggap sebagai object valid.

---

# 11. Bounded pipeline

Pipeline production:

```text
ADB Scanner
     │
     ▼
Bounded Channel
     │
     ▼
Streaming Reader
     │
     ▼
Chunker
     │
     ▼
Hash Worker
     │
     ▼
Dedup
     │
     ├── HIT ──────► Metadata
     │
     └── MISS
           │
           ▼
        Compressor
           │
           ▼
        Encryptor
           │
           ▼
        Object Writer
           │
           ▼
        Repository
```

**Tidak boleh ada tahap yang mengumpulkan seluruh file ke RAM.**

---

# 12. Urutan implementasi final

Saya akan mengunci roadmap menjadi:

### Phase 01 — Foundation

```text
Cargo Workspace
Domain
Ports
Error model
IDs
```

### Phase 02 — SQLite

```text
devices
files
snapshots
snapshot_files
chunks
file_chunks
chunk_objects
manifests
```

### Phase 03 — Chunk Core

```text
Chunk
ChunkId
ChunkHash
ChunkingStrategy
ChunkingPolicy
```

### Phase 04 — Fixed Chunker

```text
File
→ Streaming
→ 4 MiB chunks
```

### Phase 05 — BLAKE3 + Dedup

```text
Hash
→ Lookup
→ HIT/MISS
```

### Phase 06 — Object Storage

```text
UUIDv7
Temporary Object
Atomic Commit
```

### Phase 07 — Snapshot Transaction

```text
PENDING
→ VERIFY
→ COMMIT
→ COMPLETED
```

### Phase 08 — Recovery

```text
Crash detection
Resume
Rollback
Repair
```

### Phase 09 — Compression

```text
Zstd
```

### Phase 10 — Encryption

```text
HKDF
XChaCha20-Poly1305
Key versioning
```

### Phase 11 — FastCDC

```text
Fixed
   ↓
FastCDC
```

### Phase 12 — Maintenance

```text
GC
Scrubbing
Orphan detection
Cache repair
```

### Phase 13 — Android Production

```text
ADB
Device discovery
Permission handling
Scanner
Media metadata
Consistency groups
```

### Phase 14 — Cloud

```text
Remote Object Storage
Resumable Upload
Remote Dedup
Sync
```

---

## Status akhir

Saya akan menetapkan dokumen Anda sebagai:

> **Phone Backup Engine — Technical Specification v4.0 — APPROVED BASELINE**

dengan empat tambahan yang dianggap **mandatory implementation invariants**:

```text
1. file_chunks harus mempertahankan ordered chunk sequence
2. Manifest memiliki cryptographic root
3. Encryption/chunking format harus versioned
4. Object write harus atomic + crash-safe
```

Setelah empat hal tersebut dikunci, kita sudah bisa berhenti mendesain secara konseptual dan mulai membangun **kode Rust sebenarnya**, dimulai dari **Phase 01: Cargo Workspace + Domain + Ports + ID types + Error model**.
