use crate::builder::BookmarkBuilder;
use crate::domain::{BookmarkItem, BrowserType};

/// Factory for creating standard browser bookmark instances
pub struct BookmarkFactory;

impl BookmarkFactory {
    /// Create Chrome bookmark
    pub fn create_chrome(
        title: impl Into<String>,
        url: impl Into<String>,
        folder: impl Into<String>,
    ) -> BookmarkItem {
        BookmarkBuilder::new(title, url)
            .browser(BrowserType::Chrome)
            .folder(folder)
            .build()
    }

    /// Create Brave bookmark
    pub fn create_brave(
        title: impl Into<String>,
        url: impl Into<String>,
        folder: impl Into<String>,
    ) -> BookmarkItem {
        BookmarkBuilder::new(title, url)
            .browser(BrowserType::Brave)
            .folder(folder)
            .build()
    }

    /// Create Firefox bookmark
    pub fn create_firefox(
        title: impl Into<String>,
        url: impl Into<String>,
        folder: impl Into<String>,
    ) -> BookmarkItem {
        BookmarkBuilder::new(title, url)
            .browser(BrowserType::Firefox)
            .folder(folder)
            .build()
    }
}
