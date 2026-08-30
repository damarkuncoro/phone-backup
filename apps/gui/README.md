# phone-backup-gui 🖥️

The desktop dashboard for the **phone-backup** platform, built with **Tauri**, **Rust**, and **Native Web Components**.

## 🏗 Modular Architecture

The GUI is designed with a strict separation of concerns, ensuring high maintainability and zero file bloat.

### 🦀 Backend (Tauri / Rust)
Located in `src-tauri/src/`, the backend handles hardware orchestration and state management:

- **`setup.rs`**: Orchestrates system boot-up, infrastructure initialization, and background monitor spawning.
- **`state/`**: Modular application state container:
    - `storage.rs`: Strategic pattern implementation for hot-swapping storage backends (Local/S3).
    - `progress.rs`: Multi-channel progress reporting (Tauri Events + Socket.io).
- **`commands/`**: Modularized API endpoints exposed to the frontend, organized by domain (backup, device, contact, system).

### 🌐 Frontend (JavaScript / Web Components)
Located in `ui/`, the frontend follows a modular orchestrator pattern without heavy frameworks:

- **`app.js`**: The thin entry point (Orchestrator) that initializes specialized managers.
- **`core/`**: Infrastructure and management logic:
    - `NavigationManager.js`: Handles view switching and sidebar states.
    - `SearchManager.js`: Manages global file and contact search logic.
    - `EventManager.js`: Bridges events between the Rust backend and the UI components.
- **`components/`**: Atomic UI building blocks using Native Web Components:
    - `browser/`: Specialized sub-views for the data explorer (FileListView, AndroidDataView).
    - `DeviceItem.js`, `SnapshotList.js`, etc.: Encapsulated domain-specific UI elements.

## 🚀 Key Features

- **Reactive Hardware Monitoring**: Automatically detects device connection/disconnection via background threads.
- **Safe Backup Operations**: Real-time monitoring of device battery and thermal status before starting tasks.
- **Unified Data Explorer**: Seamlessly browse files, gallery items, and structured Android data (SMS, Contacts) in a single modular interface.
- **High-Performance Search**: Global FTS5-powered search integrated directly into the dashboard.

## 🛠 Development

### Prerequisites
- **Node.js**: For tailwind and asset management.
- **Rust & Tauri**: For backend and cross-platform compilation.

### Run in Development Mode
```bash
cd src-tauri
cargo tauri dev
```

---
*Clean Code. Native Speed. Modern UX.*
