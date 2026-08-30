Kode ini sudah menunjukkan **Backup Orchestrator yang cukup matang**, terutama karena sudah memiliki:

* Safety check battery & thermal
* File scanning berdasarkan policy
* Incremental backup / diff
* Resume interrupted snapshot
* Disk-space validation
* Encryption
* Structured data backup
* Deduplication berbasis hash/object store
* Retention strategy
* Snapshot lifecycle

Namun, ada beberapa **masalah desain dan potensi bug** yang sebaiknya diperbaiki sebelum masuk production.

---

# 1. Arsitektur saat ini

Alur `perform_backup()` saat ini:

```text
Device
   │
   ▼
Safety Check
   │
   ▼
Scan Files
   │
   ▼
Apply Backup Policy
   │
   ▼
Get Previous Snapshot
   │
   ▼
Resume / Create Snapshot
   │
   ▼
Compare Files
   │
   ▼
Check Disk Space
   │
   ▼
Upload Files
   │
   ├── Deduplication
   ├── Encryption
   └── Object Store
   │
   ▼
Backup Structured Data
   │
   ├── Apps
   ├── Contacts
   ├── SMS
   ├── Call Logs
   └── Other Metadata
   │
   ▼
Finalize Snapshot
   │
   ▼
Retention Strategy
```

Secara konsep ini sudah sangat cocok untuk tool backup Android yang sedang Anda bangun.

---

# 2. Masalah utama: `files_to_upload` hanya dipakai untuk disk check

Anda menghitung:

```rust
let files_to_upload: Vec<FileEntry> = manifest_files
    .iter()
    .cloned()
    .filter(|f| !already_backed_up.contains(&f.path))
    .filter(|f| {
        if let Some(prev) = previous_files.get(&f.path) {
            !(prev.size_bytes == f.size_bytes && prev.modified_at == f.modified_at)
        } else {
            true
        }
    })
    .collect();
```

Tetapi kemudian:

```rust
self.upload_files(
    id,
    &manifest_files,
    &previous_files,
    &already_backed_up,
    &mut snapshot,
    &encryption,
)?;
```

Artinya `upload_files()` menerima seluruh:

```rust
manifest_files
```

bukan:

```rust
files_to_upload
```

Kalau `upload_files()` tidak melakukan filtering ulang, maka incremental backup Anda sebenarnya tetap mencoba memproses seluruh file.

Saya sarankan:

```rust
self.upload_files(
    id,
    &files_to_upload,
    &previous_files,
    &already_backed_up,
    &mut snapshot,
    &encryption,
)?;
```

Atau lebih baik lagi, pisahkan konsep:

```text
Manifest
    │
    ▼
Backup Planner
    │
    ├── Upload
    ├── Reuse previous object
    ├── Skip
    └── Deleted
```

---

# 3. Disk space calculation belum benar untuk deduplication

Saat ini:

```rust
let total_required: u64 =
    files_to_upload.iter().map(|f| f.size_bytes).sum();
```

Masalahnya:

* File mungkin sudah ada di object store.
* File mungkin identical berdasarkan hash.
* File bisa mengalami compression.
* Encryption menambah overhead.
* Structured data belum dihitung.

Contoh:

```text
File A      2 GB
File B      2 GB
File C      2 GB

Total logical = 6 GB

Tetapi:

A dan B memiliki hash sama
C sudah ada di object store

Physical required = hampir 0 GB
```

Lebih baik buat estimator:

```rust
struct BackupEstimate {
    logical_bytes: u64,
    estimated_new_bytes: u64,
    deduplicated_bytes: u64,
    metadata_bytes: u64,
    encryption_overhead: u64,
}
```

Kemudian:

```rust
let estimate = self.estimate_backup_size(
    &files_to_upload,
    &encryption,
)?;

self.check_available_disk_space(
    estimate.estimated_new_bytes
)?;
```

---

# 4. `check_available_disk_space()` berpotensi memeriksa disk yang salah

Ini:

```rust
if let Some(disk) = disks.iter().next() {
```

sangat berbahaya.

`next()` hanya mengambil disk pertama.

Misalnya:

```text
Disk 1: Macintosh HD
Available: 500 GB

Disk 2: External Backup Disk
Available: 2 GB
```

Backup sebenarnya disimpan ke:

```text
External Backup Disk
```

Tetapi aplikasi mengecek:

```text
Macintosh HD
```

dan mengatakan storage cukup.

Lebih benar `StoragePort` mengetahui lokasi target:

```rust
pub trait StoragePort {
    fn root_path(&self) -> Result<PathBuf>;

    fn available_space(&self) -> Result<u64>;

    fn write(
        &self,
        key: &ObjectStoreKey,
        reader: &mut dyn Read,
    ) -> Result<()>;
}
```

Kemudian:

```rust
pub(crate) fn check_available_disk_space(
    &self,
    required_bytes: u64,
) -> Result<()> {
    let available = self.storage.available_space()?;

    if available < required_bytes {
        anyhow::bail!(
            "Insufficient storage space"
        );
    }

    Ok(())
}
```

Ini jauh lebih Clean Architecture karena application layer tidak perlu tahu `sysinfo`.

---

# 5. Encryption dan deduplication memiliki masalah desain penting

Di sini:

```rust
let hash = calculate_hash(&json);
```

kemudian:

```rust
let object_path =
    ObjectStoreKey::compute_object_path(&hash, &object_id);
```

baru setelah itu data dienkripsi:

```rust
data_to_write = match encryption {
    EncryptionMode::Password(pwd) =>
        EncryptionEngine::encrypt(&data_to_write, pwd)?,

    EncryptionMode::PublicKey(pk) =>
        EncryptionEngine::encrypt_with_key(&data_to_write, pk)?,

    EncryptionMode::None =>
        data_to_write,
};
```

Artinya:

```text
Plaintext
   │
   ▼
SHA256
   │
   ▼
Object ID
   │
   ▼
Encrypt
```

Ini bagus untuk deduplication, tetapi ada konsekuensi keamanan.

Jika object path:

```text
objects/ab/cd/hash.enc
```

didasarkan pada plaintext hash, maka seseorang yang memiliki akses ke metadata mungkin bisa melakukan:

```text
known plaintext hash attack
```

Misalnya data yang mudah ditebak.

---

## Desain yang lebih baik

Pisahkan:

```text
Content Hash
Storage ID
Cipher Hash
```

Contoh:

```rust
pub struct ObjectMetadata {
    pub plaintext_hash: String,
    pub ciphertext_hash: String,
    pub storage_key: String,
}
```

Untuk encryption:

```text
Plaintext
   │
   ├── SHA256 → plaintext_hash
   │
   ▼
Encrypt
   │
   ▼
Ciphertext
   │
   ├── SHA256 → ciphertext_hash
   │
   ▼
Storage Key
```

Ini lebih aman.

---

# 6. Password encryption perlu KDF

Kalau:

```rust
EncryptionEngine::encrypt(data, pwd)
```

langsung memakai password, pastikan engine tidak langsung melakukan:

```text
AES(password)
```

Harus menggunakan KDF seperti:

```text
Password
   │
   ▼
Argon2id
   │
   ├── Salt
   ├── Memory Cost
   ├── Time Cost
   └── Parallelism
   │
   ▼
Encryption Key
   │
   ▼
XChaCha20-Poly1305 / AES-256-GCM
```

Saya sarankan format object:

```text
BackupObject
│
├── magic
├── version
├── encryption_algorithm
├── kdf_algorithm
├── salt
├── nonce
├── ciphertext
└── authentication_tag
```

---

# 7. Resume logic masih memiliki race/problem

Anda melakukan:

```rust
let latest_snapshot =
    self.repository.get_latest_snapshot(id)?;
```

kemudian:

```rust
let mut snapshot =
    if let Some(incomplete) =
        self.repository.get_incomplete_snapshot(id)?
```

Masalahnya:

```text
latest_snapshot
```

bisa saja merupakan snapshot incomplete.

Kemudian:

```text
previous_files
```

dibangun dari incomplete snapshot.

Idealnya:

```text
Previous Completed Snapshot
```

dan:

```text
Current Resumable Snapshot
```

harus dipisahkan.

Saya sarankan repository API:

```rust
fn get_latest_completed_snapshot(
    &self,
    device_id: &DeviceId,
) -> Result<Option<Snapshot>>;

fn get_resumable_snapshot(
    &self,
    device_id: &DeviceId,
) -> Result<Option<Snapshot>>;
```

Kemudian:

```rust
let previous_snapshot =
    self.repository
        .get_latest_completed_snapshot(id)?;

let resumable_snapshot =
    self.repository
        .get_resumable_snapshot(id)?;
```

Ini lebih aman.

---

# 8. Snapshot harus memiliki state machine

Saat ini:

```rust
snapshot.status = SnapshotStatus::Running;
```

kemudian langsung:

```rust
snapshot.status = SnapshotStatus::Completed;
```

Saya sarankan state:

```text
Created
   │
   ▼
Preparing
   │
   ▼
Scanning
   │
   ▼
Running
   │
   ├── Interrupted
   ├── Failed
   │
   ▼
Finalizing
   │
   ▼
Completed
```

Contoh enum:

```rust
pub enum SnapshotStatus {
    Created,
    Preparing,
    Scanning,
    Running,
    Finalizing,
    Completed,
    Interrupted,
    Failed,
}
```

Lalu jangan ubah status secara bebas:

```rust
impl Snapshot {
    pub fn start(&mut self) -> Result<()> {
        match self.status {
            SnapshotStatus::Created
            | SnapshotStatus::Interrupted => {
                self.status = SnapshotStatus::Running;
                Ok(())
            }

            _ => anyhow::bail!("Invalid snapshot transition"),
        }
    }

    pub fn complete(&mut self) -> Result<()> {
        match self.status {
            SnapshotStatus::Running
            | SnapshotStatus::Finalizing => {
                self.status = SnapshotStatus::Completed;
                self.finished_at = Some(Utc::now());
                Ok(())
            }

            _ => anyhow::bail!("Invalid snapshot transition"),
        }
    }
}
```

Ini menjaga business rule tetap berada di domain.

---

# 9. Structured data deduplication sebaiknya menggunakan tipe

Saat ini:

```rust
store_structured_data(
    snapshot_id,
    data_type,
    data,
    encryption,
)
```

`data_type` adalah:

```rust
&str
```

Lebih baik gunakan enum.

```rust
pub enum StructuredDataType {
    Contacts,
    Sms,
    CallLogs,
    Applications,
    WifiNetworks,
    DeviceSettings,
}
```

Kemudian:

```rust
pub(crate) fn store_structured_data<V>(
    &self,
    snapshot_id: &SnapshotId,
    data_type: StructuredDataType,
    data: &V,
    encryption: &EncryptionMode,
) -> Result<()>
where
    V: serde::Serialize,
```

Keuntungan:

```text
❌ "contatcs"
❌ "contact"
❌ "contacts_data"

✅ StructuredDataType::Contacts
```

---

# 10. Masalah overwrite pada structured data

Ini:

```rust
if !self.storage.exists(&object_path)? {
```

bagus untuk deduplication.

Tetapi jika encryption menggunakan random nonce, dua encryption dari data sama menghasilkan ciphertext berbeda.

Misalnya:

```text
JSON A
  │
  ├── Encrypt nonce 1
  │     └── Object A
  │
  └── Encrypt nonce 2
        └── Object B
```

Namun path Anda berdasarkan plaintext hash:

```text
hash.enc
```

Akibatnya backup kedua bisa reuse ciphertext lama.

Secara functional bisa bekerja, tetapi desain perlu secara eksplisit mendefinisikan:

```text
Deduplication happens BEFORE encryption
```

atau:

```text
Deduplication happens AFTER encryption
```

Jangan tercampur.

Untuk backup tool saya lebih merekomendasikan:

```text
Content Addressable Storage
        +
Encryption Key per Backup Repository
```

Sehingga:

```text
SHA256(plaintext)
        │
        ▼
Check object exists
        │
        ▼
Encrypt once
        │
        ▼
Store immutable object
```

---

# 11. Error handling sebaiknya otomatis mark snapshot failed/interrupted

Sekarang:

```rust
self.upload_files(...)?
```

Jika gagal, function langsung return.

Snapshot mungkin tetap:

```text
Running
```

Padahal backup sudah crash.

Lebih aman:

```rust
let result = self.execute_backup(
    id,
    &manifest_files,
    &previous_files,
    &already_backed_up,
    &mut snapshot,
    &encryption,
);

match result {
    Ok(_) => {
        snapshot.status = SnapshotStatus::Completed;
        snapshot.finished_at = Some(Utc::now());

        self.repository.update_snapshot(&snapshot)?;

        Ok(snapshot)
    }

    Err(error) => {
        snapshot.status = SnapshotStatus::Interrupted;

        let _ =
            self.repository.update_snapshot(&snapshot);

        Err(error)
    }
}
```

Lebih ideal menggunakan guard.

```rust
struct SnapshotGuard<'a, R> {
    repository: &'a R,
    snapshot: &'a mut Snapshot,
    completed: bool,
}
```

Jika terjadi panic/error sebelum selesai, guard membantu memastikan status tidak tertinggal `Running`.

---

# 12. Saya sarankan memecah `perform_backup()`

Saat ini function sudah mulai menjadi orchestration besar.

Saya akan pecah menjadi:

```text
application/
│
├── backup/
│   │
│   ├── backup_service.rs
│   │
│   ├── backup_preparation.rs
│   │   ├── device validation
│   │   ├── battery
│   │   └── thermal
│   │
│   ├── backup_planner.rs
│   │   ├── manifest
│   │   ├── diff
│   │   └── upload plan
│   │
│   ├── backup_executor.rs
│   │   ├── upload
│   │   ├── resume
│   │   └── progress
│   │
│   ├── snapshot_lifecycle.rs
│   │
│   └── structured_backup.rs
│
domain/
│
├── backup/
│   ├── snapshot.rs
│   ├── backup_policy.rs
│   ├── backup_plan.rs
│   └── backup_object.rs
```

---

# 13. Desain `BackupPlan` yang saya rekomendasikan

Daripada langsung menghitung `files_to_upload`, buat domain object.

```rust
pub struct BackupPlan {
    pub upload: Vec<FileEntry>,
    pub reuse: Vec<FileReuse>,
    pub skipped: Vec<FileEntry>,
    pub deleted: Vec<DeletedFile>,
    pub logical_bytes: u64,
    pub upload_bytes: u64,
}
```

Contoh:

```text
Previous Snapshot

/photo/a.jpg
/photo/b.jpg
/photo/c.jpg


Current Device

/photo/a.jpg unchanged
/photo/b.jpg changed
/photo/d.jpg new
```

Backup plan:

```text
REUSE
/photo/a.jpg

UPLOAD
/photo/b.jpg
/photo/d.jpg

DELETED
/photo/c.jpg
```

Ini jauh lebih jelas daripada hanya:

```rust
Vec<FileEntry>
```

---

# 14. Versi orchestration yang lebih bersih

Saya menyarankan arah seperti ini:

```rust
pub fn perform_backup(
    &self,
    device_id: &DeviceId,
    encryption: EncryptionMode,
    policy: Option<BackupPolicy>,
) -> Result<Snapshot> {
    let policy = policy.unwrap_or_default();

    // Phase 1: Prepare
    let device = self.prepare_device(device_id)?;

    // Phase 2: Scan
    let manifest = self.build_manifest(
        device_id,
        &policy,
    )?;

    // Phase 3: Resolve snapshots
    let previous =
        self.repository
            .get_latest_completed_snapshot(device_id)?;

    let mut snapshot =
        self.resolve_or_create_snapshot(device_id)?;

    // Phase 4: Build backup plan
    let plan =
        self.build_backup_plan(
            &manifest,
            previous.as_ref(),
            &snapshot,
        )?;

    // Phase 5: Capacity validation
    let estimate =
        self.estimate_required_storage(
            &plan,
            &encryption,
        )?;

    self.check_available_disk_space(
        estimate.required_bytes
    )?;

    // Phase 6: Start
    snapshot.start()?;

    self.repository
        .save_snapshot(&snapshot)?;

    // Phase 7: Execute
    match self.execute_backup(
        device_id,
        &plan,
        &mut snapshot,
        &encryption,
    ) {
        Ok(_) => {
            self.backup_metadata_and_structured_data(
                device_id,
                &mut snapshot,
                &encryption,
            )?;

            snapshot.complete()?;

            self.repository
                .update_snapshot(&snapshot)?;

            self.apply_retention_strategy(
                device_id,
                &domain::KeepCountStrategy {
                    keep_limit: 10,
                },
            )?;

            Ok(snapshot)
        }

        Err(error) => {
            snapshot.interrupt()?;

            let _ =
                self.repository
                    .update_snapshot(&snapshot);

            Err(error)
        }
    }
}
```

---

# Kesimpulan

Kode Anda sudah berada di level yang bagus untuk **core backup engine**. Yang paling penting untuk diperbaiki sekarang adalah:

### Prioritas P0

1. `files_to_upload` harus benar-benar digunakan.
2. Disk space harus dicek pada storage target.
3. Pisahkan latest completed snapshot vs resumable snapshot.
4. Pastikan error tidak meninggalkan snapshot berstatus `Running`.
5. Perjelas strategi deduplication + encryption.

### Prioritas P1

6. Buat `BackupPlan`.
7. Buat snapshot state machine.
8. Gunakan enum untuk `StructuredDataType`.
9. Tambahkan storage estimator.
10. Pindahkan disk capability ke `StoragePort`.

Secara arsitektur, langkah berikutnya yang paling tepat untuk proyek backup HP Anda adalah membangun **`BackupPlanner` + `BackupPlan`**, karena itu akan menjadi pusat untuk full backup, incremental backup, resume, deduplication, deleted-file tracking, dan progress calculation.
