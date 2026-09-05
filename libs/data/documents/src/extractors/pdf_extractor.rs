use crate::domain::DocumentMetadata;

/// Specialist metadata extractor for Portable Document Format (PDF) files.
pub struct PdfExtractor;

impl PdfExtractor {
    /// Extracts metadata from PDF byte buffers without heavy native dependencies.
    pub fn extract(bytes: &[u8]) -> DocumentMetadata {
        let mut metadata = DocumentMetadata::new();

        if bytes.len() < 5 || !bytes.starts_with(b"%PDF-") {
            return metadata;
        }

        let content = String::from_utf8_lossy(bytes);

        if content.contains("/Encrypt") {
            metadata.is_password_protected = true;
        }

        if let Some(pages) = Self::extract_page_count(&content) {
            metadata.page_count = Some(pages);
        }

        if let Some(title) = Self::extract_field(&content, "/Title") {
            metadata.title = Some(title);
        }

        if let Some(author) = Self::extract_field(&content, "/Author") {
            metadata.author = Some(author);
        }

        metadata
    }

    fn extract_page_count(content: &str) -> Option<usize> {
        let mut max_count = None;
        for part in content.split("/Type /Pages") {
            if let Some(count_idx) = part.find("/Count ") {
                let after = &part[count_idx + 7..];
                let num_str: String = after
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                if let Ok(num) = num_str.parse::<usize>() {
                    max_count = Some(max_count.map_or(num, |m: usize| m.max(num)));
                }
            }
        }
        max_count
    }

    fn extract_field(content: &str, field: &str) -> Option<String> {
        if let Some(idx) = content.find(field) {
            let after = &content[idx + field.len()..];
            if let Some(start) = after.find('(') {
                if let Some(end) = after[start + 1..].find(')') {
                    let val = &after[start + 1..start + 1 + end];
                    if !val.trim().is_empty() {
                        return Some(val.trim().to_string());
                    }
                }
            }
        }
        None
    }
}
