use crate::domain::BookmarkItem;
use std::collections::BTreeMap;

/// Exporter to standard Netscape Bookmark HTML (importable in Chrome, Safari, Firefox, Brave)
pub struct NetscapeHtmlExporter;

impl NetscapeHtmlExporter {
    pub fn export(title: &str, bookmarks: &[BookmarkItem]) -> String {
        let mut out = String::new();
        out.push_str("<!DOCTYPE NETSCAPE-Bookmark-file-1>\n");
        out.push_str("<!-- This is an automatically generated file. It will be read and overwritten. Do Not Edit! -->\n");
        out.push_str("<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">\n");
        out.push_str(&format!("<TITLE>{}</TITLE>\n", title));
        out.push_str(&format!("<H1>{}</H1>\n", title));
        out.push_str("<DL><p>\n");

        // Group bookmarks by folder
        let mut grouped: BTreeMap<String, Vec<&BookmarkItem>> = BTreeMap::new();
        for b in bookmarks {
            grouped.entry(b.folder.clone()).or_default().push(b);
        }

        for (folder, items) in grouped {
            out.push_str(&format!("    <DT><H3>{}</H3>\n", escape_html(&folder)));
            out.push_str("    <DL><p>\n");
            for item in items {
                let add_date = item
                    .date_added
                    .map(|d| d.timestamp().to_string())
                    .unwrap_or_else(|| "0".to_string());

                out.push_str(&format!(
                    "        <DT><A HREF=\"{}\" ADD_DATE=\"{}\">{}</A>\n",
                    escape_html(&item.url),
                    add_date,
                    escape_html(&item.title)
                ));
            }
            out.push_str("    </DL><p>\n");
        }

        out.push_str("</DL><p>\n");
        out
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
