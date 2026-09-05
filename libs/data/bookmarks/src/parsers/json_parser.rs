use crate::domain::BookmarkItem;
use anyhow::Result;

/// Parser for JSON collections of bookmarks
pub struct BookmarkJsonParser;

impl BookmarkJsonParser {
    pub fn parse(json_str: &str) -> Result<Vec<BookmarkItem>> {
        let items: Vec<BookmarkItem> = serde_json::from_str(json_str)?;
        Ok(items)
    }
}
