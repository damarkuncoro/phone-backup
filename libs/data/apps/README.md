# phone-backup-apps 📦

Specialist crate for Android Application Package (APK / APKS / XAPK) backup, AndroidBinary XML (`AndroidManifest.xml`) parsing, and permission security auditing.

## 🏗 Architecture & Modules

- **`domain/`**: App models (`AppPackage`, `AppManifest`, `PermissionAudit`, SDK levels, version codes, dangerous permissions).
- **`parsers/`**: Zero-dependency binary XML (`AxmlParser`) decoder, string pool extractor, and zip container reader.
- **`audit/`**: Security auditor analyzing app permission risks, target SDK obsolescence, and privacy implications.
- **`exporters/`**: Security report generators (Markdown, JSON, Plaintext) and APK batch installer scripts.

## 🚀 Key Features

- **Pure Rust AXML Parser**: Extracts package name, version, and requested permissions from raw compiled APKs without requiring external tools (`aapt`/`apkanalyzer`).
- **Security & Privacy Audit**: Automatically classifies permissions into Normal, Dangerous (Camera, Location, Microphone, SMS), and Signature levels.
- **Batch Export**: Extracts installed third-party user applications directly from device into standalone `.apk` archives.
