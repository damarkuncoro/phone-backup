# phone-backup-adapter-adb 📱

A professional-grade adapter implementation for Android devices using the **Android Debug Bridge (ADB)**. This module acts as a bridge between the core backup engine and physical Android hardware.

## 🏗 Modular Architecture (DDD)

This crate follows **Domain-Driven Design (DDD)** and **Clean Architecture** principles, organized into distinct layers:

- **`gateways/`**: Implementation of Domain Ports (`DevicePort`, `ScannerPort`, etc.). Acts as the Bounded Context Gateway.
- **`parsers/`**: Anti-Corruption Layer (ACL). Responsible for translating raw ADB text output into rich Domain entities.
- **`client/`**: Low-level transport layer managing ADB binary execution, connection pooling, and lifecycle.
- **`scripts/`**: Centralized repository of optimized Android shell script templates.

## 🚀 Advanced Features

- **Hybrid MediaStore Scraper**: Combined filesystem `find` with `content query` to extract rich metadata (GPS coordinates, image dimensions, creation dates) **instantly** without downloading files.
- **Zero-Copy Streaming I/O**: Direct data transfer from device to storage via memory streams (`exec-out`), bypassing slow temporary files and extending SSD life.
- **Reactive Device Monitor**: Real-time hardware detection using `adb track-devices`. Notifies the engine immediately when a device is connected or removed.
- **Resilience Engine**: Built-in **Exponential Backoff Retry** strategy to handle transient USB connection instabilities.
- **Hardware Safety Guard**: Real-time monitoring of device **Battery Level** and **Thermal State** to protect user hardware during intensive backup tasks.

## 🛠 Design Patterns Used

- **Builder Pattern**: Fluent configuration for `AdbClient` (timeouts, custom paths).
- **Factory Pattern**: Centralized creation of gateways via `AdbGatewayFactory`.
- **Facade Pattern**: Simplified high-level API via `AdbAdapter` for easy integration.
- **Command Builder**: Type-safe construction of complex ADB commands.

## ⚙️ Requirements

- **ADB Binary**: Must be installed on the host system.
- **Android Version**: Supports Android 5.0 (API 21) through Android 15+.
- **Permissions**: Requires ADB Debugging enabled on the target device.

---
*Built for speed, reliability, and developer happiness.*
