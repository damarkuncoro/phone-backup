# phone-backup-contacts 📇

Specialist crate for address book contact extraction, vCard (v2.1, v3.0, v4.0) parsing, deduplication, and contact graph synchronization.

## 🏗 Architecture & Modules

- **`domain/`**: Contact entities (`Contact`, `PhoneNumber`, `EmailAddress`, `PostalAddress`, avatar blobs, custom fields).
- **`parsers/`**: Android `content://com.android.contacts` parser and multi-version vCard RFC 6350 parser.
- **`exporters/`**: Universal `.vcf` vCard exporter, CSV spreadsheet generator, and JSON document builder.
- **`dedup/`**: Fuzzy matching and identity resolution merging duplicate contact entries across multiple SIMs/accounts.

## 🚀 Key Features

- **Multi-Version vCard Compliance**: Lossless export into standard `.vcf` files for immediate import on iOS, Android, and Webmail.
- **Photo / Avatar Preservation**: Base64 decoding and binary extraction of embedded contact profile pictures.
- **Organization & Group Indexing**: Preserves company names, job titles, nicknames, and contact group tags.
