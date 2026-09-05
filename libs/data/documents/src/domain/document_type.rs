use serde::{Deserialize, Serialize};

/// High-level document specialization and format category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    /// Portable Document Format (.pdf)
    Pdf,
    /// Spreadsheets and Data Sheets (.xlsx, .xls, .csv, .ods)
    Spreadsheet,
    /// Word processing, rich text, and articles (.docx, .doc, .odt, .rtf)
    WordProcessing,
    /// Slide decks and presentation bundles (.pptx, .ppt, .odp)
    Presentation,
    /// Digital publications and electronic books (.epub, .mobi, .azw3)
    EBook,
    /// Plaintext, notes, markdown, and config files (.txt, .md, .json, .xml, .yaml)
    TextOrCode,
    /// Other unclassified document formats
    Other,
}

impl DocumentType {
    /// Infers the document category from a file path or extension.
    pub fn from_path(path: &str) -> Self {
        let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
        match ext.as_str() {
            "pdf" => Self::Pdf,
            "xlsx" | "xls" | "csv" | "ods" | "tsv" => Self::Spreadsheet,
            "docx" | "doc" | "odt" | "rtf" | "pages" => Self::WordProcessing,
            "pptx" | "ppt" | "odp" | "key" => Self::Presentation,
            "epub" | "mobi" | "azw3" | "fb2" => Self::EBook,
            "txt" | "md" | "markdown" | "json" | "xml" | "yaml" | "yml" | "log" | "ini" => {
                Self::TextOrCode
            }
            _ => Self::Other,
        }
    }
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pdf => write!(f, "PDF Document"),
            Self::Spreadsheet => write!(f, "Spreadsheet"),
            Self::WordProcessing => write!(f, "Word Document"),
            Self::Presentation => write!(f, "Presentation"),
            Self::EBook => write!(f, "E-Book"),
            Self::TextOrCode => write!(f, "Text / Markdown"),
            Self::Other => write!(f, "Other Document"),
        }
    }
}
