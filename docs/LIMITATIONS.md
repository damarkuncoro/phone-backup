# ⚠️ Known Limitations

This document outlines what **phone-backup** (and most ADB-based tools) cannot back up due to Android's security model and technical constraints.

## 1. Private App Data (`/data/data/`)
Android isolates each application's internal database and preferences. 
- **What's missing**: Chat history (WhatsApp/Telegram), game saves, and logged-in sessions.
- **Reason**: Access to these folders requires **Root** privileges. 
- **Exception**: Some apps allow backups via the legacy `adb backup` command, but this is deprecated and increasingly blocked by app developers.

## 2. System Settings & OS Configuration
- **What's missing**: System-wide settings, Wi-Fi passwords, Bluetooth pairings, and home screen layouts.
- **Reason**: These are stored in protected system partitions and databases managed by the Android System Server.

## 3. Secure Element & Biometrics
- **What's missing**: Fingerprints, Face ID data, and hardware-backed cryptographic keys (Keystore/Keymaster).
- **Reason**: This data never leaves the hardware's **TEE (Trusted Execution Environment)** or Secure Element. It is physically impossible to extract.

## 4. DRM-Protected Content
- **What's missing**: Offline downloads from Netflix, Spotify, Disney+, etc.
- **Reason**: These files are encrypted with keys tied to the device's hardware ID and are often stored in hidden or protected paths.

## 5. Cloud-Only Data
- **What's missing**: Files in Google Photos, Google Drive, or iCloud that are **not** locally cached on the device.
- **Reason**: If the file isn't on the phone's storage, the scanner cannot see it.

## 6. Device Firmware & IMEI (EFS)
- **What's missing**: The Operating System itself, bootloader, modem firmware, and the EFS partition (containing IMEI and network calibration).
- **Reason**: Requires low-level partition access (EDL mode or custom recovery like TWRP).

---

### 💡 Recommendation
For a "Complete" migration, we recommend:
1.  Use **phone-backup** for all Photos, Videos, Documents, and APKs.
2.  Use the built-in **Google/Manufacturer Cloud Backup** for system settings and Wi-Fi.
3.  Use the **In-App Backup** features for apps like WhatsApp (to Google Drive).
