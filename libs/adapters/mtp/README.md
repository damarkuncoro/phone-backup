# phone-backup-adapter-mtp 🔌

A Media Transfer Protocol (MTP) adapter for the phone-backup engine. This library provides plug-and-play support for Android devices without requiring USB Debugging or Developer Mode.

## Features

- **Plug & Play**: Automatically detects Android devices mounted as storage on macOS and Linux without requiring Developer Mode or USB Debugging.
- **Native USB & Mount Discovery**: Directly communicates with MTP USB endpoints via `mtp-rs` as well as filesystem mounts.
- **macOS Conflict Resolver**: Automatically detects and terminates interfering macOS background daemons (`ptpcamerad`, `PTPCamera`, `mscamerad`) that hold exclusive device locks.
- **Port Implementation**: Fully implements `ports::DevicePort` and `ports::ScannerPort`.

## Usage & Diagnostics

Test connected MTP devices via CLI:
```bash
# List MTP devices
cargo run --bin phone-backup -- --adapter mtp devices

# Diagnostic scan of physical phone
cargo run --example real_device_test --package phone-backup-adapter-mtp
```

## Verified Real Hardware

- **Infinix NOTE 30 (Infinix X6833B)**: Verified 100% functional on macOS (USB MTP session open, storage partition discovery, recursive object/folder listing for DCIM, Pictures, Downloads, Documents).
- **Xiaomi Redmi Note 12 Pro 5G**: Verified functional via ADB and MTP.

## Limitations

MTP is inherently limited compared to ADB:
- No support for system-level structured data (SMS/Call Logs).
- Slower metadata scanning compared to direct ADB queries.
- Restricted access to application-specific private directories.
