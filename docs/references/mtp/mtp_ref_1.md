Untuk **Phone Backup Engine**, akses **MTP (Media Transfer Protocol)** sangat penting sebagai jalur backup ketika **ADB tidak tersedia**. MTP cocok untuk mengakses file pengguna melalui USB seperti foto, video, dokumen, musik, dan folder yang diizinkan Android.

## 1. Posisi MTP dalam Phone Backup Engine

Saya sarankan arsitekturnya seperti ini:

```text
                    ┌─────────────────────┐
                    │   Phone Backup UI   │
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │   Backup Service    │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
        ┌─────▼─────┐    ┌─────▼─────┐   ┌─────▼─────┐
        │ ADB Source│    │ MTP Source│   │ Other     │
        │ Provider  │    │ Provider  │   │ Sources   │
        └─────┬─────┘    └─────┬─────┘   └───────────┘
              │                │
              ▼                ▼
        Android Debug      USB MTP/PTP
              │                │
              └────────┬───────┘
                       │
              ┌────────▼────────┐
              │ Unified Scanner │
              └────────┬────────┘
                       │
              ┌────────▼────────┐
              │ Backup Pipeline │
              │ Hash/Compress/  │
              │ Encrypt/Store   │
              └─────────────────┘
```

**MTP jangan dijadikan bagian dari Scanner secara langsung.** Lebih baik dibuat sebagai `DevicePort` atau `DataProviderPort`.

---

# 2. Apa yang Bisa Diakses Melalui MTP?

Secara umum MTP memungkinkan backup:

```text
/storage/emulated/0/

├── DCIM/
│   ├── Camera/
│   └── Screenshots/
│
├── Pictures/
├── Movies/
├── Music/
├── Download/
├── Documents/
│
├── Android/
│   └── media/
│       └── ...
│
└── folder aplikasi tertentu yang diekspos Android
```

Namun MTP **tidak bisa dianggap sebagai full filesystem access**.

Biasanya tidak bisa mengakses secara bebas:

```text
/data/
/system/
/vendor/
/proc/
/sys/
```

Dan pada Android modern, akses ke:

```text
Android/data/
Android/obb/
```

bisa dibatasi tergantung versi Android dan implementasi vendor.

---

# 3. Arsitektur Rust yang Saya Rekomendasikan

Karena Phone Backup Engine Anda sudah menggunakan pendekatan Clean Architecture/DDD, buat abstraction seperti ini.

## Domain Port

```rust
#[async_trait::async_trait]
pub trait DevicePort: Send + Sync {
    async fn list_devices(&self) -> anyhow::Result<Vec<Device>>;
    async fn get_device(&self, id: &DeviceId) -> anyhow::Result<Device>;
}
```

Lalu:

```rust
#[async_trait::async_trait]
pub trait ScannerPort: Send + Sync {
    async fn scan(
        &self,
        device: &DeviceId,
    ) -> anyhow::Result<Vec<FileEntry>>;
}
```

Untuk MTP, implementasi infrastructure:

```text
infrastructure/
└── mtp/
    ├── mtp_client.rs
    ├── mtp_device_provider.rs
    ├── mtp_scanner.rs
    ├── mtp_file_reader.rs
    ├── mtp_capabilities.rs
    └── error.rs
```

---

# 4. Interface MTP yang Lebih Baik

Saya justru menyarankan interface generik supaya ADB dan MTP bisa menggunakan pipeline backup yang sama.

```rust
#[async_trait::async_trait]
pub trait DataProviderPort {
    async fn list_roots(&self) -> anyhow::Result<Vec<StorageRoot>>;

    async fn scan(
        &self,
        root: &StorageRoot,
    ) -> anyhow::Result<Vec<FileEntry>>;

    async fn open(
        &self,
        file: &FileEntry,
    ) -> anyhow::Result<Box<dyn AsyncRead + Unpin + Send>>;
}
```

Implementasi:

```text
DataProviderPort
       │
       ├── AdbDataProvider
       │
       ├── MtpDataProvider
       │
       └── FilesystemDataProvider
```

Backup engine tidak perlu tahu apakah file berasal dari ADB atau MTP.

```rust
let stream = provider.open(&file).await?;

backup_engine
    .backup_stream(stream, &file)
    .await?;
```

Ini sangat cocok untuk desain pipeline Anda sebelumnya.

---

# 5. MTP Device Discovery

Alur discovery:

```text
USB Device Connected
        │
        ▼
USB Enumeration
        │
        ▼
Detect MTP Interface
        │
        ▼
Open MTP Session
        │
        ▼
Get Device Information
        │
        ▼
Get Storage IDs
        │
        ▼
Enumerate Objects
```

Informasi device:

```rust
pub struct MtpDeviceInfo {
    pub manufacturer: String,
    pub model: String,
    pub serial: Option<String>,
    pub version: Option<String>,
    pub vendor_extension: Option<String>,
}
```

Storage:

```rust
pub struct MtpStorage {
    pub id: u32,
    pub name: String,
    pub capacity_bytes: u64,
    pub free_space_bytes: u64,
}
```

---

# 6. Struktur File di MTP

Berbeda dengan filesystem biasa, MTP bekerja dengan konsep **Object Handle**.

Contoh:

```text
Storage ID: 0x00010001

Handle: 1
Name: DCIM
Parent: ROOT

Handle: 2
Name: Camera
Parent: 1

Handle: 3
Name: IMG_001.jpg
Parent: 2
```

Karena itu, scanner harus membangun path virtual.

```rust
pub struct MtpObject {
    pub handle: u32,
    pub parent_handle: u32,
    pub storage_id: u32,

    pub name: String,
    pub size_bytes: u64,

    pub modified_at: Option<DateTime<Utc>>,

    pub is_directory: bool,
}
```

Kemudian path resolver:

```rust
/storage/emulated/0/DCIM/Camera/IMG_001.jpg
```

dibangun dari relasi:

```text
Object
   │
   ▼
Parent Object
   │
   ▼
Parent Object
   │
   ▼
Storage Root
```

---

# 7. Scanner MTP

Saya menyarankan scanner tidak langsung membuat seluruh object tree di RAM untuk device dengan puluhan ribu file.

Gunakan traversal.

```text
Storage
   │
   ├── DCIM
   │     ├── Camera
   │     │      ├── IMG001
   │     │      └── IMG002
   │     │
   │     └── Screenshots
   │
   ├── Download
   │
   └── Documents
```

Pseudo-code:

```rust
async fn scan_directory(
    session: &mut MtpSession,
    parent: u32,
    path: &Path,
    sender: Sender<FileEntry>,
) -> Result<()> {

    let objects =
        session.list_objects(parent).await?;

    for object in objects {

        let object_path =
            path.join(&object.name);

        if object.is_directory {

            scan_directory(
                session,
                object.handle,
                &object_path,
                sender.clone(),
            ).await?;

        } else {

            sender.send(
                FileEntry {
                    path: object_path,
                    name: object.name,
                    size_bytes: object.size_bytes,
                    ..
                }
            ).await?;
        }
    }

    Ok(())
}
```

Tetapi implementasi production sebaiknya menggunakan iterative traversal agar tidak terkena masalah stack pada struktur folder yang sangat dalam.

---

# 8. Streaming File dari MTP

Jangan lakukan:

```text
MTP
 ↓
Download seluruh file ke RAM
 ↓
Compress
 ↓
Encrypt
 ↓
Store
```

Lebih baik:

```text
MTP Device
    │
    │ chunks
    ▼
┌──────────────┐
│ Buffer 256KB │
└──────┬───────┘
       ▼
┌──────────────┐
│ Hash SHA-256 │
└──────┬───────┘
       ▼
┌──────────────┐
│ Compression  │
└──────┬───────┘
       ▼
┌──────────────┐
│ Encryption   │
└──────┬───────┘
       ▼
 Object Storage
```

Contoh konsep:

```rust
let mut reader =
    mtp_provider.open(&file).await?;

let mut buffer = vec![0u8; 256 * 1024];

loop {

    let bytes =
        reader.read(&mut buffer).await?;

    if bytes == 0 {
        break;
    }

    hasher.update(&buffer[..bytes]);

    compressor.write(
        &buffer[..bytes]
    ).await?;
}
```

Ini jauh lebih aman untuk video besar 5–20 GB.

---

# 9. Incremental Backup MTP

Karena MTP tidak selalu memberikan semua metadata filesystem seperti inode, pendekatan incremental sebaiknya menggunakan:

```text
Path
+
Size
+
Modified Time
+
Content Hash
```

Level cepat:

```text
path sama
AND
size sama
AND
modified_at sama
```

→ kemungkinan tidak berubah.

Jika ingin lebih aman:

```text
Quick Hash
```

Misalnya:

```text
SHA256(
    first 1 MB
    +
    last 1 MB
    +
    file_size
)
```

Jika berubah:

```text
Full SHA-256
```

Arsitektur:

```text
MTP File
    │
    ▼
Metadata Comparison
    │
    ├── Unchanged → Skip
    │
    ├── Suspicious
    │       │
    │       ▼
    │   Quick Hash
    │
    └── New/Changed
            │
            ▼
        Full Backup
```

---

# 10. MTP + ADB Hybrid Mode

Menurut saya ini adalah desain terbaik untuk Phone Backup Engine Anda.

```text
              Android Phone
                    │
          ┌─────────┴─────────┐
          │                   │
         ADB                 MTP
          │                   │
          ▼                   ▼
    Deep Access         User Files
          │                   │
          └─────────┬─────────┘
                    ▼
             Unified Backup
```

Priority:

```text
1. ADB
2. MTP
3. Filesystem Mount
```

Atau berdasarkan kategori:

| Data               | Provider           |
| ------------------ | ------------------ |
| DCIM               | MTP / ADB          |
| Pictures           | MTP                |
| Movies             | MTP                |
| Music              | MTP                |
| Download           | MTP                |
| Documents          | MTP                |
| App-specific media | ADB → MTP fallback |
| System metadata    | ADB                |
| Deep Android paths | ADB                |

---

# 11. Tambahkan Capability Detection

Jangan hardcode bahwa setiap Android bisa diakses dengan cara yang sama.

```rust
pub struct DeviceCapabilities {
    pub adb_available: bool,
    pub mtp_available: bool,

    pub can_access_dcim: bool,
    pub can_access_downloads: bool,

    pub can_access_android_data: bool,
    pub can_access_android_media: bool,

    pub supports_multiple_storage: bool,
}
```

Saat device terhubung:

```text
Device Connected
       │
       ▼
Capability Detector
       │
       ├── ADB Available?
       │
       ├── MTP Available?
       │
       ├── Storage Count
       │
       ├── Accessible Roots
       │
       ▼
Backup Strategy Resolver
```

Kemudian:

```rust
pub enum BackupTransport {
    Adb,
    Mtp,
    Filesystem,
}
```

Resolver:

```rust
fn resolve_transport(
    capabilities: &DeviceCapabilities,
) -> BackupTransport {

    if capabilities.adb_available {
        return BackupTransport::Adb;
    }

    if capabilities.mtp_available {
        return BackupTransport::Mtp;
    }

    BackupTransport::Filesystem
}
```

---

# 12. Struktur Project yang Saya Rekomendasikan

```text
phone-backup/
│
├── domain/
│   ├── device.rs
│   ├── storage.rs
│   ├── file_entry.rs
│   ├── snapshot.rs
│   └── backup_policy.rs
│
├── application/
│   ├── backup_service.rs
│   ├── scan_service.rs
│   ├── device_discovery_service.rs
│   └── transport_resolver.rs
│
├── ports/
│   ├── device_port.rs
│   ├── scanner_port.rs
│   ├── data_provider_port.rs
│   └── storage_port.rs
│
├── infrastructure/
│   │
│   ├── adb/
│   │   ├── adb_client.rs
│   │   └── adb_provider.rs
│   │
│   ├── mtp/
│   │   ├── mtp_client.rs
│   │   ├── mtp_session.rs
│   │   ├── mtp_provider.rs
│   │   ├── mtp_scanner.rs
│   │   └── mtp_reader.rs
│   │
│   ├── storage/
│   │   ├── local_storage.rs
│   │   └── object_storage.rs
│   │
│   └── repository/
│       └── sqlite_repository.rs
│
└── cli/
```

---

## Rekomendasi untuk Engine Anda

Untuk project **Phone Backup Engine** yang sedang Anda bangun, saya akan menetapkan:

```text
PRIMARY TRANSPORT
ADB

FALLBACK TRANSPORT
MTP

FILE ACCESS MODEL
Streaming

INCREMENTAL STRATEGY
Metadata → Quick Hash → Full Hash

ARCHITECTURE
Transport-agnostic Provider

FILE STORAGE
Chunk-based
```

**MTP sebaiknya menjadi provider resmi kedua**, bukan sekadar fallback kecil. Banyak pengguna tidak mengaktifkan USB Debugging, sehingga MTP kemungkinan akan menjadi jalur utama untuk backup file pengguna.

Kalau Anda ingin, tahap berikutnya saya bisa langsung merancang **MTP Provider production-ready dalam Rust**, termasuk:

* `MtpDataProvider`
* USB device discovery
* recursive/iterative object scanner
* Object Handle → virtual path resolver
* streaming reader
* error recovery ketika kabel dicabut
* reconnect mechanism
* integrasi dengan `BackupService` dan pipeline chunk/compression/encryption yang sudah Anda desain.
