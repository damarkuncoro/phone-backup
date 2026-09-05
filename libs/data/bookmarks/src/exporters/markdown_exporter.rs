use crate::domain::BookmarkItem;
use std::collections::BTreeMap;

/// Exporter for bookmarks to organized Markdown
pub struct BookmarkMarkdownExporter;

impl BookmarkMarkdownExporter {
    pub fn export(title: &str, bookmarks: &[BookmarkItem]) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", title));
        out.push_str(&format!("Total bookmarks: {}\n\n", bookmarks.len()));

        let mut grouped: BTreeMap<String, Vec<&BookmarkItem>> = BTreeMap::new();
        for b in bookmarks {
            grouped.entry(b.folder.clone()).or_default().push(b);
        }

        for (folder, items) in grouped {
            out.push_str(&format!("## 📁 {}\n\n", folder));
            for item in items {
                let clean_title = if item.title.trim().is_empty() {
                    &item.url
                } else {
                    &item.title
                };
                out.push_str(&format!("- [{}]({})\n", clean_title, item.url));
            }
            out.push('\n');
        }

        out
    }
}
