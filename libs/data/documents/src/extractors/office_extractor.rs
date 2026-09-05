use crate::domain::DocumentMetadata;
use std::io::{Cursor, Read};
use zip::ZipArchive;

/// Specialist metadata extractor for Office Open XML (DOCX, XLSX, PPTX) and EPUB archives.
pub struct OfficeExtractor;

impl OfficeExtractor {
    /// Extracts embedded properties from OOXML and EPUB ZIP containers.
    pub fn extract(bytes: &[u8]) -> DocumentMetadata {
        let mut metadata = DocumentMetadata::new();
        let reader = Cursor::new(bytes);
        let mut archive = match ZipArchive::new(reader) {
            Ok(a) => a,
            Err(_) => return metadata,
        };

        if let Ok(mut core_file) = archive.by_name("docProps/core.xml") {
            let mut xml = String::new();
            if core_file.read_to_string(&mut xml).is_ok() {
                if let Some(title) = Self::extract_xml_tag(&xml, "dc:title") {
                    metadata.title = Some(title);
                }
                if let Some(author) = Self::extract_xml_tag(&xml, "dc:creator") {
                    metadata.author = Some(author);
                }
                if let Some(created) = Self::extract_xml_tag(&xml, "dcterms:created") {
                    metadata.created_date = Some(created);
                }
            }
        }

        if let Ok(mut app_file) = archive.by_name("docProps/app.xml") {
            let mut xml = String::new();
            if app_file.read_to_string(&mut xml).is_ok() {
                if let Some(pages) = Self::extract_xml_tag(&xml, "Pages") {
                    if let Ok(p) = pages.parse::<usize>() {
                        metadata.page_count = Some(p);
                    }
                }
                if let Some(words) = Self::extract_xml_tag(&xml, "Words") {
                    if let Ok(w) = words.parse::<usize>() {
                        metadata.word_count = Some(w);
                    }
                }
            }
        }

        metadata
    }

    fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
        let open_tag = format!("<{}>", tag);
        let close_tag = format!("</{}>", tag);
        if let Some(start) = xml.find(&open_tag) {
            let after = &xml[start + open_tag.len()..];
            if let Some(end) = after.find(&close_tag) {
                let text = after[..end].trim();
                if !text.is_empty() {
                    return Some(text.to_string());
                }
            }
        }
        None
    }
}
