# phone-backup-adapter-agent 📶

The Wireless Companion Agent adapter for the phone-backup engine. This library implements the `ports::DevicePort` and related interfaces to communicate with the Android Companion App over the network.

## Features

- **Wireless Discovery**: Detects devices running the Phone Backup Companion App on the same Wi-Fi network.
- **Custom Protocol**: Implements the `AgentHandshake` and `AgentHeartbeat` protocols for secure wireless communication.
- **Binary Streaming**: Optimized binary pipeline for transferring files and structured data (SMS/Contacts) without physical cables.
- **Async Communication**: Built on top of `tokio` for non-blocking network I/O.

## Architecture

This is a **Transport Adapter** in the Hexagonal Architecture. It allows the Core Engine to interact with remote Android devices as if they were connected via USB, abstracting the network complexity away from the business logic.

## Supported Protocols

- Wi-Fi gRPC / WebSocket (Phase 43+)
- QR-Code based pairing (Upcoming)
