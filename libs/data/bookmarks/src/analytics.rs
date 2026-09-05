use crate::domain::{BookmarkItem, BookmarkStats, BrowserType};
use std::collections::{HashMap, HashSet};

/// Domain service for bookmark analytics, ranking, and search
pub struct BookmarkAnalytics;

impl BookmarkAnalytics {
    /// Compute statistics and domain popularity rankings
    pub fn compute_stats(bookmarks: &[BookmarkItem]) -> BookmarkStats {
        let mut domain_counts: HashMap<String, usize> = HashMap::new();
        let mut browser_dist: HashMap<BrowserType, usize> = HashMap::new();
        let mut folders: HashSet<String> = HashSet::new();

        for b in bookmarks {
            folders.insert(b.folder.clone());
            *browser_dist.entry(b.browser).or_insert(0) += 1;
            let host = b.domain_host();
            if !host.is_empty() {
                *domain_counts.entry(host).or_insert(0) += 1;
            }
        }

        let mut top_domains: Vec<(String, usize)> = domain_counts.into_iter().collect();
        top_domains.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        BookmarkStats {
            total_bookmarks: bookmarks.len(),
            total_folders: folders.len(),
            top_domains,
            browser_distribution: browser_dist,
        }
    }

    /// Filter bookmarks by browser, folder, or keyword query
    pub fn filter_bookmarks(
        bookmarks: Vec<BookmarkItem>,
        browser: Option<BrowserType>,
        folder: Option<&str>,
        query: Option<&str>,
    ) -> Vec<BookmarkItem> {
        bookmarks
            .into_iter()
            .filter(|b| {
                if let Some(br) = browser {
                    if b.browser != br {
                        return false;
                    }
                }
                if let Some(f) = folder {
                    if !b.folder.eq_ignore_ascii_case(f) {
                        return false;
                    }
                }
                if let Some(q) = query {
                    let q_lower = q.to_lowercase();
                    if !b.title.to_lowercase().contains(&q_lower)
                        && !b.url.to_lowercase().contains(&q_lower)
                    {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
