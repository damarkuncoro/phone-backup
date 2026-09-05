use crate::domain::{DocumentItem, DocumentMetadata, DocumentType};
use crate::extractors::{OfficeExtractor, PdfExtractor, TextExtractor};
use domain::FileEntry;

/// Domain service coordinating document analysis and metadata extraction.
pub struct DocumentAnalyzer;

impl DocumentAnalyzer {
    /// Analyzes document bytes based on file path and extension.
    pub fn analyze_bytes(path: &str, bytes: &[u8]) -> DocumentMetadata {
        let doc_type = DocumentType::from_path(path);
        match doc_type {
            DocumentType::Pdf => PdfExtractor::extract(bytes),
            DocumentType::WordProcessing
            | DocumentType::Spreadsheet
            | DocumentType::Presentation
            | DocumentType::EBook => OfficeExtractor::extract(bytes),
            DocumentType::TextOrCode => TextExtractor::extract(bytes),
            DocumentType::Other => DocumentMetadata::default(),
        }
    }

    /// Analyzes a FileEntry and produces an enriched DocumentItem.
    pub fn analyze_entry(entry: &FileEntry, bytes: Option<&[u8]>) -> DocumentItem {
        let metadata = if let Some(b) = bytes {
            Self::analyze_bytes(&entry.path, b)
        } else {
            DocumentMetadata::default()
        };

        DocumentItem::from_file_entry(entry, metadata)
    }

    /// Enriches an existing DocumentItem with extracted metadata from bytes.
    pub fn enrich_document(mut doc: DocumentItem, bytes: &[u8]) -> DocumentItem {
        doc.metadata = Self::analyze_bytes(&doc.path, bytes);
        doc
    }

    /// Filters and sorts document items by specific category or size.
    pub fn filter_documents(
        items: Vec<DocumentItem>,
        target_type: Option<DocumentType>,
        min_size: Option<u64>,
    ) -> Vec<DocumentItem> {
        items
            .into_iter()
            .filter(|doc| {
                if let Some(t) = target_type {
                    if doc.doc_type != t {
                        return false;
                    }
                }
                if let Some(min) = min_size {
                    if doc.size_bytes < min {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
