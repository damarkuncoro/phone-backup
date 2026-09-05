use crate::domain::BookmarkItem;
use anyhow::Result;

/// Exporter for bookmarks to JSON
pub struct BookmarkJsonExporter;

impl BookmarkJsonExporter {
    pub fn export(bookmarks: &[BookmarkItem]) -> Result<String> {
        let json = serde_json::to_string_pretty(bookmarks)?;
        Ok(json)
    }
}
