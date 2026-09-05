# phone-backup-wifi 📶

Specialist crate for Wi-Fi profile extraction, security categorization (WPA2/WPA3/WEP), and automated QR code connection generation.

## 🏗 Architecture & Modules

- **`domain/`**: Wi-Fi network profile models (`WifiNetwork`, `SecurityType`, SSID, Pre-shared Key, Hidden flag).
- **`parsers/`**: Android `WifiConfigStore.xml`, legacy `wpa_supplicant.conf`, and custom backup parsers.
- **`exporters/`**: Formatter generating readable text summaries, JSON exports, and WPA configuration files.
- **`qr/`**: SVG & ASCII QR Code generator encoding `WIFI:S:<SSID>;T:<TYPE>;P:<PASS>;;` protocol for instantaneous phone camera scanning.

## 🚀 Key Features

- **Instant QR Pairing**: Generates standard Wi-Fi QR codes directly in terminal (ASCII) or image formats for quick device reconnection.
- **Multi-Source Config Parsing**: Parses Android 8 through Android 15 Wi-Fi configuration schemas.
- **Network Security Auditing**: Identifies open/unencrypted SSIDs and outdated WEP/WPA protocols.
