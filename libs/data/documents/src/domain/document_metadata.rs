use serde::{Deserialize, Serialize};

/// Detailed metadata extracted from parsed document streams.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    /// Document title embedded in header/core properties.
    pub title: Option<String>,
    /// Creator or primary author of the document.
    pub author: Option<String>,
    /// Estimated or exact page count (PDF, Word).
    pub page_count: Option<usize>,
    /// Number of sheets or tables (Spreadsheets).
    pub sheet_count: Option<usize>,
    /// Word count (Text/Docx).
    pub word_count: Option<usize>,
    /// Original creation or revision timestamp.
    pub created_date: Option<String>,
    /// Whether the document requires a password or encryption key to open.
    pub is_password_protected: bool,
    /// Extracted text snippet for indexing and instant search preview.
    pub text_snippet: Option<String>,
}

impl DocumentMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn with_page_count(mut self, pages: usize) -> Self {
        self.page_count = Some(pages);
        self
    }

    pub fn with_sheet_count(mut self, sheets: usize) -> Self {
        self.sheet_count = Some(sheets);
        self
    }

    pub fn with_word_count(mut self, words: usize) -> Self {
        self.word_count = Some(words);
        self
    }

    pub fn with_password_protected(mut self, protected: bool) -> Self {
        self.is_password_protected = protected;
        self
    }

    pub fn with_text_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.text_snippet = Some(snippet.into());
        self
    }
}
