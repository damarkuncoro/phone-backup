use crate::domain::{DocumentItem, DocumentMetadata, DocumentType};
use chrono::{DateTime, Utc};

/// Fluent builder for constructing `DocumentItem` entities.
#[derive(Default)]
pub struct DocumentItemBuilder {
    path: String,
    name: String,
    doc_type: Option<DocumentType>,
    size_bytes: u64,
    modified_at: Option<DateTime<Utc>>,
    metadata: DocumentMetadata,
}

impl DocumentItemBuilder {
    pub fn new(path: impl Into<String>) -> Self {
        let p = path.into();
        let name = p.rsplit('/').next().unwrap_or(&p).to_string();
        Self {
            path: p,
            name,
            doc_type: None,
            size_bytes: 0,
            modified_at: None,
            metadata: DocumentMetadata::default(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_type(mut self, doc_type: DocumentType) -> Self {
        self.doc_type = Some(doc_type);
        self
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size_bytes = size;
        self
    }

    pub fn with_modified(mut self, modified: DateTime<Utc>) -> Self {
        self.modified_at = Some(modified);
        self
    }

    pub fn with_metadata(mut self, metadata: DocumentMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn build(self) -> DocumentItem {
        let doc_type = self
            .doc_type
            .unwrap_or_else(|| DocumentType::from_path(&self.path));
        let modified_at = self.modified_at.unwrap_or_else(Utc::now);

        DocumentItem::new(
            self.path,
            self.name,
            doc_type,
            self.size_bytes,
            modified_at,
            self.metadata,
        )
    }
}
