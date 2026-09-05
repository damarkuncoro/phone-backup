# phone-backup-whatsapp 💬

Specialist crate for WhatsApp and WhatsApp Business message backup parsing, media tree indexing, and encrypted database handling.

## 🏗 Architecture & Modules

- **`domain/`**: WhatsApp models (`WhatsAppChat`, `WhatsAppMessage`, media types, PTT voice messages, system messages).
- **`parsers/`**:
  - `txt_parser.rs`: Universal parser for exported `.txt` chat files (supports Android and iOS date/bracket formats).
  - `discovery.rs`: Resolves legacy (`/sdcard/WhatsApp/Media`) and Android 11+ Scoped Storage (`/Android/media/com.whatsapp/WhatsApp/Media`) directory layouts.
- **`exporters/`**: Interactive HTML chat visualizer, JSON archiver, and media catalog generator.

## 🚀 Key Features

- **Dual OS Export Parsing**: Seamlessly parses exported chats generated from either Android (`[dd/MM/yy, hh:mm:ss] Name: Msg`) or iOS (`[dd/MM/yyyy, hh:mm:ss a] Name: Msg`).
- **Scoped Storage Auto-Discovery**: Automatically traverses Android 11, 12, 13, 14, and 15 WhatsApp media repositories.
- **PTT Voice & Video Indexing**: Accurately indexes Push-to-Talk audio (.opus) and video notes.
