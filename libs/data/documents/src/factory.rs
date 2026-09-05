use crate::builder::DocumentItemBuilder;
use crate::domain::{DocumentItem, DocumentMetadata, DocumentType};
use chrono::Utc;

/// Factory for generating pre-populated sample and specialized document instances.
pub struct DocumentFactory;

impl DocumentFactory {
    /// Factory for creating a PDF document item with page count and title.
    pub fn create_pdf(
        path: impl Into<String>,
        title: impl Into<String>,
        pages: usize,
        size_bytes: u64,
    ) -> DocumentItem {
        let meta = DocumentMetadata::new()
            .with_title(title)
            .with_page_count(pages);

        DocumentItemBuilder::new(path)
            .with_type(DocumentType::Pdf)
            .with_size(size_bytes)
            .with_modified(Utc::now())
            .with_metadata(meta)
            .build()
    }

    /// Factory for creating a spreadsheet document item.
    pub fn create_spreadsheet(
        path: impl Into<String>,
        title: impl Into<String>,
        sheets: usize,
        size_bytes: u64,
    ) -> DocumentItem {
        let meta = DocumentMetadata::new()
            .with_title(title)
            .with_sheet_count(sheets);

        DocumentItemBuilder::new(path)
            .with_type(DocumentType::Spreadsheet)
            .with_size(size_bytes)
            .with_modified(Utc::now())
            .with_metadata(meta)
            .build()
    }

    /// Factory for creating a text/markdown document item with snippet.
    pub fn create_text(
        path: impl Into<String>,
        snippet: impl Into<String>,
        word_count: usize,
        size_bytes: u64,
    ) -> DocumentItem {
        let meta = DocumentMetadata::new()
            .with_text_snippet(snippet)
            .with_word_count(word_count);

        DocumentItemBuilder::new(path)
            .with_type(DocumentType::TextOrCode)
            .with_size(size_bytes)
            .with_modified(Utc::now())
            .with_metadata(meta)
            .build()
    }
}
