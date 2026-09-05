use crate::builder::BookmarkBuilder;
use crate::domain::{BookmarkItem, BrowserType};

/// Parser for standard Netscape Bookmark HTML files
pub struct NetscapeHtmlParser;

impl NetscapeHtmlParser {
    pub fn parse(html_content: &str, browser: BrowserType) -> Vec<BookmarkItem> {
        let mut bookmarks = Vec::new();
        let mut current_folder = "Bookmarks".to_string();

        for line in html_content.lines() {
            let trimmed = line.trim();

            // Detect Folder Header <H3 ...>FolderName</H3>
            if let Some(pos) = trimmed.find("<H3") {
                if let Some(end_tag) = trimmed[pos..].find('>') {
                    let rest = &trimmed[pos + end_tag + 1..];
                    if let Some(close) = rest.find("</H3>") {
                        let folder_name = &rest[..close];
                        if !folder_name.is_empty() {
                            current_folder = folder_name.trim().to_string();
                        }
                    }
                }
            }

            // Detect Bookmark <A HREF="url" ...>Title</A>
            if let Some(pos) = trimmed.find("<A ") {
                let rest = &trimmed[pos..];
                if let Some(href_pos) = rest.find("HREF=\"") {
                    let after_href = &rest[href_pos + 6..];
                    if let Some(quote_pos) = after_href.find('"') {
                        let url = &after_href[..quote_pos];
                        if let Some(tag_end) = rest.find('>') {
                            let after_tag = &rest[tag_end + 1..];
                            let title = if let Some(close_a) = after_tag.find("</A>") {
                                &after_tag[..close_a]
                            } else {
                                url
                            };

                            let item = BookmarkBuilder::new(title.trim(), url.trim())
                                .folder(&current_folder)
                                .browser(browser)
                                .build();

                            bookmarks.push(item);
                        }
                    }
                }
            }
        }

        bookmarks
    }
}
