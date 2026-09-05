use crate::domain::{BookmarkItem, BrowserType};
use chrono::{DateTime, Utc};

/// Fluent Builder for constructing BookmarkItem instances
#[derive(Debug, Default, Clone)]
pub struct BookmarkBuilder {
    id: Option<String>,
    title: String,
    url: String,
    folder: String,
    browser: Option<BrowserType>,
    date_added: Option<DateTime<Utc>>,
    date_modified: Option<DateTime<Utc>>,
    favicon: Option<String>,
}

impl BookmarkBuilder {
    pub fn new(title: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            id: None,
            title: title.into(),
            url: url.into(),
            folder: "Bookmarks".to_string(),
            browser: None,
            date_added: None,
            date_modified: None,
            favicon: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn folder(mut self, folder: impl Into<String>) -> Self {
        self.folder = folder.into();
        self
    }

    pub fn browser(mut self, browser: BrowserType) -> Self {
        self.browser = Some(browser);
        self
    }

    pub fn date_added(mut self, dt: DateTime<Utc>) -> Self {
        self.date_added = Some(dt);
        self
    }

    pub fn date_modified(mut self, dt: DateTime<Utc>) -> Self {
        self.date_modified = Some(dt);
        self
    }

    pub fn favicon(mut self, favicon: impl Into<String>) -> Self {
        self.favicon = Some(favicon.into());
        self
    }

    pub fn build(self) -> BookmarkItem {
        let id = self
            .id
            .unwrap_or_else(|| format!("{:x}", md5_hash(&self.url)));
        BookmarkItem {
            id,
            title: self.title,
            url: self.url,
            folder: self.folder,
            browser: self.browser.unwrap_or(BrowserType::Generic),
            date_added: self.date_added,
            date_modified: self.date_modified,
            favicon: self.favicon,
        }
    }
}

fn md5_hash(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
