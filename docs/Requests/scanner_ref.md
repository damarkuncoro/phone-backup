Kode `AdbScannerRepository` ini sudah memiliki desain yang cukup baik untuk **scanner file Android berbasis ADB**. Pendekatan utamanya adalah menggabungkan dua sumber data:

1. **MediaStore Android** → metadata media yang diketahui sistem Android.
2. **Filesystem scan (`find`)** → file yang benar-benar ditemukan pada folder target.

Kemudian hasilnya di-*deduplicate* menggunakan `path`.

## Alur kode saat ini

```text
scan()
 │
 ├── scan_mediastore()
 │    ├── query image
 │    └── query video
 │
 ├── HashMap<path, FileEntry>
 │
 ├── resolve_roots()
 │
 ├── filesystem scan
 │
 ├── merge filesystem entries
 │
 └── return Vec<FileEntry>
```

Ini cocok untuk proyek **Android Backup Tool** karena MediaStore saja tidak cukup, sementara filesystem scan saja biasanya memiliki metadata yang lebih terbatas.

---

# Hal yang sudah bagus

### 1. Default roots

```rust
const DEFAULT_SCAN_ROOTS: &[&str] = &[
    "/storage/emulated/0/DCIM",
    "/storage/emulated/0/Pictures",
    "/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media",
    "/storage/emulated/0/WhatsApp/Media",
];
```

Ini sudah memperhitungkan beberapa variasi lokasi WhatsApp.

### 2. Deduplication menggunakan HashMap

```rust
let mut entries_map: HashMap<String, FileEntry> =
    media_entries.into_iter()
        .map(|f| (f.path.clone(), f))
        .collect();
```

Lalu:

```rust
entries_map
    .entry(fs_file.path.clone())
    .or_insert(fs_file);
```

Artinya MediaStore diprioritaskan jika path sudah ditemukan sebelumnya.

### 3. Repository tidak mengetahui detail parsing

Ini bagus secara Clean Architecture:

```text
AdbScannerRepository
        │
        ├── AdbClient
        │
        ├── AndroidScripts
        │
        └── MediaParser
```

Repository berfungsi sebagai orchestration layer.

---

# Masalah utama yang perlu diperbaiki

## 1. Error dari ADB saat ini diabaikan

Saat ini:

```rust
if let Ok(out) = self.client.shell(...) {
```

Jika perangkat disconnect atau ADB gagal, hasilnya hanya menjadi `Vec` kosong.

Contoh:

```text
ADB disconnected
↓
scan_mediastore()
↓
return []
↓
scan()
↓
return Ok([])
```

Ini berbahaya karena aplikasi backup dapat menganggap:

> "Tidak ada file"

Padahal sebenarnya:

> "Tidak bisa membaca device"

### Sebaiknya

Pisahkan antara:

```text
No files found
```

dan:

```text
Scan failed
```

Contoh:

```rust
fn scan_mediastore(&self, device_id: &DeviceId) -> Result<Vec<FileEntry>> {
    let mut all_media = Vec::new();

    let image_out = self.client.shell(
        &device_id.0,
        &AndroidScripts::query_mediastore("image"),
    )?;

    all_media.extend(
        MediaParser::parse_mediastore(device_id, &image_out)
    );

    let video_out = self.client.shell(
        &device_id.0,
        &AndroidScripts::query_mediastore("video"),
    )?;

    all_media.extend(
        MediaParser::parse_mediastore(device_id, &video_out)
    );

    Ok(all_media)
}
```

---

# 2. Filesystem scan sebaiknya jangan seluruhnya silent failure

Saat ini:

```rust
if let Ok(stdout) = self.client.shell(&device_id.0, &script) {
```

Jika folder tidak dapat diakses atau `find` gagal, Anda kehilangan informasi.

Untuk backup tool, saya menyarankan membuat `ScanResult`.

```rust
pub struct ScanResult {
    pub files: Vec<FileEntry>,
    pub warnings: Vec<ScanWarning>,
}
```

Contoh:

```rust
pub struct ScanWarning {
    pub source: ScanSource,
    pub message: String,
}
```

```rust
pub enum ScanSource {
    MediaStoreImages,
    MediaStoreVideos,
    FileSystem,
}
```

Sehingga UI bisa menampilkan:

```text
✓ Images scanned: 2,341
✓ Videos scanned: 420
⚠ WhatsApp Media folder inaccessible
```

Daripada langsung gagal seluruh scan.

---

# 3. Hasil HashMap tidak deterministic

Bagian:

```rust
entries_map.into_values().collect()
```

Urutan hasil `HashMap` tidak stabil.

Hari ini:

```text
A.jpg
C.jpg
B.jpg
```

Scan berikutnya:

```text
B.jpg
A.jpg
C.jpg
```

Ini dapat menyulitkan:

* snapshot comparison
* testing
* progress calculation
* UI
* incremental backup

### Sebaiknya sort

Misalnya berdasarkan path:

```rust
let mut entries: Vec<FileEntry> =
    entries_map.into_values().collect();

entries.sort_by(|a, b| a.path.cmp(&b.path));

Ok(entries)
```

Atau lebih baik menggunakan `BTreeMap`.

```rust
use std::collections::BTreeMap;
```

Kemudian:

```rust
let mut entries_map: BTreeMap<String, FileEntry> = ...
```

---

# 4. Prioritas metadata sebaiknya lebih eksplisit

Saat ini:

```rust
entries_map.entry(fs_file.path.clone()).or_insert(fs_file);
```

Artinya:

```text
MediaStore wins
Filesystem ignored
```

Tetapi kadang filesystem bisa memiliki informasi lebih baru, misalnya:

* file size
* modified timestamp
* permission
* filesystem path

Saya menyarankan membuat fungsi merge.

```rust
fn merge_file_entries(
    mediastore: FileEntry,
    filesystem: FileEntry,
) -> FileEntry {
    FileEntry {
        id: mediastore.id,
        path: mediastore.path,
        name: filesystem.name,
        size_bytes: filesystem.size_bytes,
        modified_at: filesystem.modified_at,
        mime_type: mediastore.mime_type
            .or(filesystem.mime_type),
        permissions: filesystem.permissions,
        hash_sha256: None,
        media_info: mediastore.media_info,
    }
}
```

Dengan demikian arsitektur lebih jelas:

```text
MediaStore
   │
   ├── MIME type
   ├── Media metadata
   ├── Duration
   └── Dimensions

Filesystem
   │
   ├── Size
   ├── Modified time
   ├── Permissions
   └── Actual path

          ↓

     FileEntry Merger
```

---

# Versi repository yang saya rekomendasikan

```rust
use crate::client::AdbClient;
use crate::parsers::media_parser::MediaParser;
use crate::scripts::AndroidScripts;

use anyhow::{Context, Result};
use domain::{DeviceId, FileEntry};

use std::collections::BTreeMap;

const DEFAULT_SCAN_ROOTS: &[&str] = &[
    "/storage/emulated/0/DCIM",
    "/storage/emulated/0/Pictures",
    "/storage/emulated/0/Movies",
    "/storage/emulated/0/Download",
    "/storage/emulated/0/Android/media/com.whatsapp/WhatsApp/Media",
    "/storage/emulated/0/WhatsApp/Media",
];

#[derive(Clone)]
pub struct AdbScannerRepository {
    client: AdbClient,
}

impl AdbScannerRepository {
    pub fn new(client: AdbClient) -> Self {
        Self { client }
    }

    fn resolve_roots(
        &self,
        provided_roots: Vec<String>,
    ) -> Vec<String> {
        if provided_roots.is_empty() {
            DEFAULT_SCAN_ROOTS
                .iter()
                .map(|root| root.to_string())
                .collect()
        } else {
            provided_roots
        }
    }

    fn scan_mediastore(
        &self,
        device_id: &DeviceId,
    ) -> Result<Vec<FileEntry>> {
        let mut all_media = Vec::new();

        let image_script =
            AndroidScripts::query_mediastore("image");

        let image_out = self
            .client
            .shell(&device_id.0, &image_script)
            .context("Failed to query MediaStore images")?;

        all_media.extend(
            MediaParser::parse_mediastore(
                device_id,
                &image_out,
            ),
        );

        let video_script =
            AndroidScripts::query_mediastore("video");

        let video_out = self
            .client
            .shell(&device_id.0, &video_script)
            .context("Failed to query MediaStore videos")?;

        all_media.extend(
            MediaParser::parse_mediastore(
                device_id,
                &video_out,
            ),
        );

        Ok(all_media)
    }

    fn scan_filesystem(
        &self,
        device_id: &DeviceId,
        roots: &[String],
    ) -> Result<Vec<FileEntry>> {
        let script =
            AndroidScripts::find_files(roots);

        let stdout = self
            .client
            .shell(&device_id.0, &script)
            .context("Failed to scan Android filesystem")?;

        Ok(
            MediaParser::parse_filesystem_scan(
                device_id,
                &stdout,
            ),
        )
    }

    pub fn scan(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
    ) -> Result<Vec<FileEntry>> {
        let media_entries =
            self.scan_mediastore(device_id)?;

        let scan_roots =
            self.resolve_roots(roots);

        let filesystem_entries =
            self.scan_filesystem(
                device_id,
                &scan_roots,
            )?;

        let mut entries =
            BTreeMap::<String, FileEntry>::new();

        for media in media_entries {
            entries.insert(
                media.path.clone(),
                media,
            );
        }

        for file in filesystem_entries {
            entries
                .entry(file.path.clone())
                .or_insert(file);
        }

        Ok(
            entries
                .into_values()
                .collect(),
        )
    }
}
```

---

# Arsitektur yang lebih baik untuk Phase Scanner

Saya menyarankan scanner jangan hanya satu `AdbScannerRepository`.

Pisahkan berdasarkan source.

```text
infrastructure/
│
├── adb/
│   │
│   ├── AdbClient
│   │
│   ├── scanner/
│   │   │
│   │   ├── MediaStoreScanner
│   │   ├── FileSystemScanner
│   │   ├── DownloadScanner
│   │   ├── WhatsAppScanner
│   │   └── ScannerAggregator
│   │
│   └── scripts/
│       ├── media_store.rs
│       ├── filesystem.rs
│       └── device.rs
```

Domain:

```text
domain/
│
├── entities/
│   ├── device.rs
│   ├── file_entry.rs
│   └── snapshot.rs
│
├── repositories/
│   └── device_scanner.rs
│
└── value_objects/
    ├── device_id.rs
    ├── file_path.rs
    └── file_hash.rs
```

---

# Interface domain yang saya sarankan

```rust
pub trait DeviceScanner {
    fn scan(
        &self,
        device_id: &DeviceId,
        roots: Vec<String>,
    ) -> Result<Vec<FileEntry>>;
}
```

Lalu:

```rust
pub struct AdbScannerRepository {
    mediastore_scanner: MediaStoreScanner,
    filesystem_scanner: FileSystemScanner,
}
```

Sehingga repository hanya menjadi coordinator.

```text
Application Layer
       │
       ▼
DeviceScanner Trait
       │
       ▼
AdbScannerRepository
       │
       ├──────────────┐
       ▼              ▼
MediaStore       FileSystem
Scanner          Scanner
       │              │
       └──────┬───────┘
              ▼
         File Merger
              │
              ▼
        Vec<FileEntry>
```

---

## Rekomendasi berikutnya

Untuk tool backup Android Anda, setelah `AdbScannerRepository`, saya sarankan modul berikutnya adalah:

```text
PHASE Scanner
├── Device Discovery
├── Device Information
├── MediaStore Scanner
├── Filesystem Scanner
├── File Deduplicator
├── File Metadata Merger
├── File Classifier
└── Scan Progress
```

Kemudian baru:

```text
PHASE Backup
├── Backup Planner
├── Storage Estimator
├── Copy Engine
├── Hash Verifier
├── Resume Support
└── Snapshot Manager
```

Kode Anda sudah berada di arah yang benar. **Perubahan paling penting sekarang adalah memisahkan scanner MediaStore dan filesystem, lalu menambahkan `ScanResult` + warning/error agar backup tidak salah menganggap device kosong ketika ADB sebenarnya gagal.**
