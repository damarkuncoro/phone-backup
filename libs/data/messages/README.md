# phone-backup-messages 💬

Specialist crate for SMS and MMS message parsing, conversation thread reconstruction, OTP code extraction, and multi-format export.

## 🏗 Architecture & Modules

- **`domain/`**: SMS & MMS data models (`SmsMessage`, `ConversationThread`, `MessageType`, timestamps, read receipts).
- **`parsers/`**: Android `content://sms` provider parser, XML backup parser, and raw PDU decoders.
- **`exporters/`**: Formatter generating universal SMS Backup XML, HTML chat visualizers, JSON, and CSV.
- **`intelligence/`**: Smart heuristic engine for 4-8 digit OTP / 2FA code detection and spam/transactional categorization.

## 🚀 Key Features

- **Chronological Threading**: Automatically groups disparate incoming and outgoing SMS into continuous conversation threads by phone number.
- **Universal XML Backup**: Exports industry-standard XML files compatible with third-party SMS restore apps.
- **HTML Chat Export**: Creates interactive chat logs with styled sender/recipient speech bubbles.
