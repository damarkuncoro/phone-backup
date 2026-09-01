# phone-backup-adapter-mtp 🔌

A Media Transfer Protocol (MTP) adapter for the phone-backup engine. This library provides plug-and-play support for Android devices without requiring USB Debugging or Developer Mode.

## Features

- **Plug & Play**: Automatically detects Android devices mounted as storage on macOS and Linux.
- **Root Discovery**: Intelligently identifies "Internal Storage" and "DCIM" paths.
- **Port Implementation**: Fully implements `ports::DevicePort` and `ports::ScannerPort`.
- **Platform Specific**: 
  - macOS: Support for `/Volumes` based MTP mounts.
  - Linux: Support for GVFS/MTP mounts (Phase 36+).

## Architecture

This is an **I/O Adapter** in the Hexagonal Monorepo. It acts as a bridge between the physical MTP filesystem and the Core Engine's abstract port definitions.

## Limitations

MTP is inherently limited compared to ADB:
- No support for system-level structured data (SMS/Call Logs).
- Slower metadata scanning compared to direct ADB queries.
- Restricted access to application-specific private directories.
