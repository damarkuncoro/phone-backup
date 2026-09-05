# phone-backup-documents 📄

Specialist crate for document discovery, PDF metadata extraction, Office document classification, and full-text index preprocessing.

## 🏗 Architecture & Modules

- **`domain/`**: Document models (`DocumentItem`, `DocumentType`, page counts, author, title, file size, mime).
- **`parsers/`**: PDF header metadata parser, OpenXML/Office (.docx, .xlsx, .pptx) container inspector, and plaintext tokenizer.
- **`exporters/`**: Catalog generator producing Markdown tables, CSV summaries, and JSON metadata trees.

## 🚀 Key Features

- **Format Classification**: Detects PDFs, Word docs, Excel spreadsheets, PowerPoint presentations, EPUB books, and Markdown notes.
- **Metadata Extraction**: Reads title, page counts, author, and creation timestamps directly from file headers without full disk load.
- **Deduplication Tagging**: Integrates with content chunker to avoid storing redundant document revisions.
