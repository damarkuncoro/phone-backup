# phone-backup-adapter-ios 🍏

Native adapter implementation for Apple iOS and iPadOS devices using **Apple File Conduit (AFC)** and `libimobiledevice` protocols.

## 🏗 Architecture & Modules

- **`gateways/`**: Implements domain ports (`DevicePort`, `ScannerPort`) for Apple hardware.
- **`client/`**: Low-level bindings and subprocess manager interfacing with `ideviceinfo`, `idevicepair`, `afcclient`, and usbmuxd.
- **`parsers/`**: Translates Apple plist schemas, device UDID descriptors, and iOS storage structures into standard domain entities.

## 🚀 Key Features

- **AFC Native File Streaming**: Direct file extraction across standard iOS sandboxed directories (DCIM Camera Roll, Books, App Documents).
- **Zero-Password Hardware Identification**: Queries device model, iOS build version, battery health, and pairing state via usbmuxd.
- **Auto-Pairing & Trust Management**: Detects device trust state (`Trust This Computer`) and guides the user when pairing certificates are required.

## ⚙️ Requirements

- `libimobiledevice` and `usbmuxd` installed on host system (e.g., `brew install libimobiledevice`).
- Target iOS / iPadOS device connected via Lightning or USB-C.
