use super::document_metadata::DocumentMetadata;
use super::document_type::DocumentType;
use chrono::{DateTime, Utc};
use domain::FileEntry;
use serde::{Deserialize, Serialize};

/// Document Domain Entity / Aggregate representing an analyzed document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentItem {
    pub path: String,
    pub name: String,
    pub doc_type: DocumentType,
    pub size_bytes: u64,
    pub modified_at: DateTime<Utc>,
    pub metadata: DocumentMetadata,
}

impl DocumentItem {
    pub fn new(
        path: impl Into<String>,
        name: impl Into<String>,
        doc_type: DocumentType,
        size_bytes: u64,
        modified_at: DateTime<Utc>,
        metadata: DocumentMetadata,
    ) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            doc_type,
            size_bytes,
            modified_at,
            metadata,
        }
    }

    pub fn from_file_entry(entry: &FileEntry, metadata: DocumentMetadata) -> Self {
        let doc_type = DocumentType::from_path(&entry.path);
        Self {
            path: entry.path.clone(),
            name: entry.name.clone(),
            doc_type,
            size_bytes: entry.size_bytes,
            modified_at: entry.modified_at,
            metadata,
        }
    }
}
