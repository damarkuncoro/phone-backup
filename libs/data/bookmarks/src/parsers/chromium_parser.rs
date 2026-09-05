use crate::builder::BookmarkBuilder;
use crate::domain::{BookmarkItem, BrowserType};
use serde_json::Value;

/// Recursive parser for Chromium / Chrome / Brave / Edge Bookmarks JSON trees
pub struct ChromiumBookmarksParser;

impl ChromiumBookmarksParser {
    pub fn parse(json_content: &str, browser: BrowserType) -> Vec<BookmarkItem> {
        let mut bookmarks = Vec::new();
        if let Ok(root) = serde_json::from_str::<Value>(json_content) {
            if let Some(roots) = root.get("roots").and_then(|r| r.as_object()) {
                for (_root_name, node) in roots {
                    Self::traverse_node(node, "", browser, &mut bookmarks);
                }
            }
        }
        bookmarks
    }

    fn traverse_node(
        node: &Value,
        current_folder: &str,
        browser: BrowserType,
        out: &mut Vec<BookmarkItem>,
    ) {
        let node_type = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let name = node.get("name").and_then(|n| n.as_str()).unwrap_or("");

        if node_type == "folder" {
            let next_folder = if current_folder.is_empty() {
                name.to_string()
            } else if !name.is_empty() {
                format!("{}/{}", current_folder, name)
            } else {
                current_folder.to_string()
            };

            if let Some(children) = node.get("children").and_then(|c| c.as_array()) {
                for child in children {
                    Self::traverse_node(child, &next_folder, browser, out);
                }
            }
        } else if node_type == "url" {
            if let Some(url) = node.get("url").and_then(|u| u.as_str()) {
                let mut builder = BookmarkBuilder::new(name, url)
                    .folder(current_folder)
                    .browser(browser);

                if let Some(id_str) = node.get("id").and_then(|i| i.as_str()) {
                    builder = builder.id(id_str);
                }

                out.push(builder.build());
            }
        }
    }
}
