//! Specialist Document Intelligence, Metadata Extractor, and Archival Engine.
//!
//! Provides clean DDD architecture, metadata extraction for PDF, Office (DOCX/XLSX/PPTX),
//! E-Books, and Text documents with Builder and Factory design patterns.

pub mod analyzer;
pub mod builder;
pub mod domain;
pub mod extractors;
pub mod factory;

pub use analyzer::DocumentAnalyzer;
pub use builder::DocumentItemBuilder;
pub use domain::{DocumentItem, DocumentMetadata, DocumentType};
pub use extractors::{OfficeExtractor, PdfExtractor, TextExtractor};
pub use factory::DocumentFactory;
