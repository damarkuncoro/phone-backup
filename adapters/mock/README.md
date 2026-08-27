# phone-backup-adapter-mock

A simulation adapter suite for the phone-backup platform.

## 🧪 Purpose

The Mock Adapter allows developers to test the full backup pipeline without requiring a physical Android device or the ADB toolchain. It provides:

- **Seeded Devices**: Simulated "Pixel" devices with hardcoded hardware info.
- **Virtual Filesystem**: A set of predictable mock files (Photos, PDFs, Text) with stable metadata.
- **Fake Data**: Simulated SMS and Contacts for testing structured data backup.
- **Zero Latency**: Instantaneous responses for UI and integration testing.

## 🏃 Usage

Used as the default adapter in the CLI for demonstration purposes:
```bash
phone-backup --adapter mock devices
```
