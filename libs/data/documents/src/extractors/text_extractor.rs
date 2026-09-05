use crate::domain::DocumentMetadata;

/// Specialist metadata extractor for plaintext, markdown, CSV, and code documents.
pub struct TextExtractor;

impl TextExtractor {
    /// Extracts word count, line statistics, and search snippet preview.
    pub fn extract(bytes: &[u8]) -> DocumentMetadata {
        let text = String::from_utf8_lossy(bytes);
        let trimmed = text.trim();

        let word_count = trimmed.split_whitespace().count();

        let snippet = if trimmed.len() > 180 {
            let mut s: String = trimmed.chars().take(180).collect();
            s.push_str("...");
            s
        } else {
            trimmed.to_string()
        };

        DocumentMetadata::new()
            .with_word_count(word_count)
            .with_text_snippet(snippet)
    }
}
