# 💡 Feature Requests & Future Ideas

This document tracks planned features, community requests, and brainstormed ideas for the **phone-backup** platform.

## 🚀 Priority: High

- [ ] **Desktop GUI (Phase 24)**: A user-friendly interface for non-technical users.
- [ ] **Automatic Periodic Backups**: A background service/daemon that triggers backups when a device is connected.
- [ ] **Asymmetric Encryption**: Allow users to use a Public Key for encryption so the password isn't needed during the backup process (only for restore).

## 📱 Device Connectivity

- [ ] **MTP Adapter**: Direct file access via Media Transfer Protocol without requiring ADB/Developer Mode.
- [ ] **iOS Adapter**: Basic photo and contact backup for iPhones using libimobiledevice.
- [ ] **Wireless Auto-Discovery**: Automatic discovery of devices over the same WiFi network using ADB-over-WiFi.

## ☁️ Storage & Cloud

- [ ] **Google Drive / Dropbox Adapters**: Native integration with consumer cloud providers.
- [ ] **Encrypted Index (SQLite Encryption)**: Encrypt the local metadata database itself for maximum privacy.
- [ ] **Remote Repository Sync**: Ability to sync the local `objects/` directory with a remote one (Rsync-like).

## 🧠 Intelligence & UX

- [ ] **Duplicate Finder UI**: A tool to list and manage duplicated files across different devices.
- [ ] **Media Viewer**: A built-in photo and video viewer to preview files directly from the encrypted repository.
- [ ] **Web Dashboard**: A self-hosted web interface to manage backups from any browser.
- [ ] **Delta Compression**: For very large files (like databases), store only the changed blocks instead of the whole file.

## 🛡 Security & Compliance

- [ ] **Hardware Key Support**: Integration with Yubikey or Ledger for managing encryption keys.
- [ ] **Audit Logs**: A tamper-proof log of every backup, restore, and deletion event.
- [ ] **Multi-User Support**: Roles and permissions for shared backup servers.

---
*Have an idea? Feel free to open a PR or add it to this list!*
