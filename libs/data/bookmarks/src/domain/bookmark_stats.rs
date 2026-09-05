use super::BrowserType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Aggregated statistics of browser bookmarks
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BookmarkStats {
    pub total_bookmarks: usize,
    pub total_folders: usize,
    pub top_domains: Vec<(String, usize)>,
    pub browser_distribution: HashMap<BrowserType, usize>,
}
