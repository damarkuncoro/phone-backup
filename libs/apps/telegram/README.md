# phone-backup-telegram ✈️

Specialist crate for Telegram chat archive indexing, desktop/mobile export parsing, and media cataloging.

## 🏗 Architecture & Modules

- **`domain/`**: Telegram chat entities (`TelegramChat`, `TelegramMessage`, `MediaType`, stickers, voice notes, video messages).
- **`parsers/`**: Ingestion engine for official Telegram Data Export archives (`result.json`) and local cached media structures.
- **`exporters/`**: Formatter producing standalone HTML chat logs, JSON trees, and media indexes.

## 🚀 Key Features

- **Rich Message Classification**: Accurately differentiates text messages, voice notes, video notes (round videos), stickers, and document attachments.
- **Media Path Resolver**: Links exported chat logs to physical media chunks on disk.
- **Offline HTML Chat Viewer**: Renders conversations with modern Telegram-style user interface and interactive media players.
