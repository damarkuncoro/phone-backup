# phone-backup-adapter-adb

A concrete adapter implementation of `DevicePort`, `ScannerPort`, `AppProviderPort`, and `DataProviderPort` using the **Android Debug Bridge (ADB)**.

## 🛠 Functionality

- **Device Discovery**: Lists connected Android devices via USB or WiFi.
- **Remote Filesystem Scanning**: Uses optimized shell commands (`find`, `stat`) to quickly inventory large filesystems.
- **Data Streaming**: Efficiently pulls and pushes files using binary-safe transfers.
- **App Extraction**: Extracts APK files for backup and supports remote installation for cloning.
- **Content Provider Querying**: Directly queries Android system providers for SMS, Contacts, and Call Logs.

## ⚙️ Requirements

This adapter requires the `adb` binary to be installed and available in the system `PATH`. It is compatible with devices running Android 5.0 (API 21) and above.
