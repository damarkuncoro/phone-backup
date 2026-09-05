# Phone Backup - Desktop GUI (Tauri Backend) 🖥️

Modern, cross-platform Desktop GUI backend powered by **Tauri v2** and **Rust**. Connects the reactive frontend (Vue/TypeScript/Tailwind) directly to the high-performance Rust core backup engine.

## 🏗 Architecture & Modules

Organized cleanly into specialized Tauri commands and state handlers:

- **`commands/`**: Command handlers exposed to the frontend via Tauri IPC (`device`, `backup`, `restore`, `datavault`, `stats`, `settings`, `troubleshoot`).
- **`state/`**: Thread-safe application state managing active `BackupService`, SQLite repositories, event buses, and cancellation tokens.
- **`events/`**: Real-time event streaming bridge emitting backup progress, transfer speeds, and hardware disconnect alerts to the UI.
- **`setup.rs`**: Application initialization, window customization, tray icon management, and background worker initialization.

## 🚀 Key Features

- **Real-Time Transfer Monitoring**: Live bandwidth speedometers, deduplication ratio indicators, and per-file progress meters.
- **Interactive Device Troubleshooting**: Built-in wizard for resolving USB MTP locks (`icdd`, `ptpcamerad`) and ADB authorization issues.
- **Zero-Latency Data Vault Explorer**: Instant search and browsing of backed up contacts, SMS threads, call logs, photos, and documents.
- **System Tray & Background Scheduling**: Low-footprint tray daemon for automated incremental backups.

## ⚙️ Development

```bash
# Run desktop application in development mode
npm run tauri dev

# Build production executable
npm run tauri build
```
