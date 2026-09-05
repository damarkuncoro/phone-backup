# phone-backup-notes 📝

Specialist crate for mobile note taking databases, quick memos, checklist parsing, and Markdown / Plaintext note conversion.

## 🏗 Architecture & Modules

- **`domain/`**: Note models (`NoteItem`, `ChecklistItem`, title, content, tags, folder, creation & modification timestamps).
- **`parsers/`**: Note export ingestion (Google Keep JSON / Takeout, generic vendor note SQLite/XML dumps).
- **`exporters/`**: Markdown exporter with YAML frontmatter, standalone HTML document generator, and JSON archiver.

## 🚀 Key Features

- **Markdown-First Export**: Translates raw mobile notes into clean `.md` files with checklist syntax (`- [x] done`), suitable for Obsidian, Logseq, and Notion.
- **Tag & Folder Hierarchy**: Retains user categorization tags and notebook folder structures.
- **Attachment Indexing**: Tracks image and audio attachments linked within notes.
