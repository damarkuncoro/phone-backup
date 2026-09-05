use super::BrowserType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Aggregate Root representing a browser bookmark or reading list entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookmarkItem {
    pub id: String,
    pub title: String,
    pub url: String,
    pub folder: String,
    pub browser: BrowserType,
    pub date_added: Option<DateTime<Utc>>,
    pub date_modified: Option<DateTime<Utc>>,
    pub favicon: Option<String>,
}

impl BookmarkItem {
    /// Extract domain host name from URL
    pub fn domain_host(&self) -> String {
        if let Some(pos) = self.url.find("://") {
            let rest = &self.url[pos + 3..];
            let domain = rest.split('/').next().unwrap_or("").split(':').next().unwrap_or("");
            return domain.to_string();
        }
        self.url.split('/').next().unwrap_or("").to_string()
    }
}
