# phone-backup-calls 📞

Specialist crate for Android and iOS telephony call history extraction, duration analytics, and universal format export.

## 🏗 Architecture & Modules

- **`domain/`**: Call models (`CallLog`, `CallType`, duration, timestamp, contact matching).
- **`parsers/`**: Android `calllog` content provider parser and CSV/XML ingest engines.
- **`exporters/`**: Multi-format serializers generating HTML summaries, Call-Logs-Backup XML, JSON, and CSV.
- **`analytics/`**: Aggregator computing total incoming/outgoing talk time, missed call ratios, and top contact frequency.

## 🚀 Key Features

- **Full Call Type Classification**: Incoming, Outgoing, Missed, Voicemail, Rejected, and Blocked calls.
- **Universal XML Export**: Compatible with standard Android Call Logs Backup & Restore applications.
- **Top Caller Metrics**: Generates rich frequency distribution and talk duration leaderboards.
