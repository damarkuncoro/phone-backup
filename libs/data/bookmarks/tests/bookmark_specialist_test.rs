use phone_backup_bookmarks::{
    BookmarkAnalytics, BookmarkBuilder, BookmarkFactory, BookmarkJsonExporter, BookmarkJsonParser,
    BookmarkMarkdownExporter, BrowserType, ChromiumBookmarksParser, NetscapeHtmlExporter,
    NetscapeHtmlParser,
};

#[test]
fn test_builder_and_factory() {
    let bm1 = BookmarkBuilder::new("Rust Programming Language", "https://www.rust-lang.org")
        .browser(BrowserType::Chrome)
        .folder("Tech/Rust")
        .build();

    assert_eq!(bm1.title, "Rust Programming Language");
    assert_eq!(bm1.domain_host(), "www.rust-lang.org");
    assert_eq!(bm1.folder, "Tech/Rust");
    assert_eq!(bm1.browser, BrowserType::Chrome);

    let bm2 = BookmarkFactory::create_brave(
        "GitHub",
        "https://github.com",
        "Development",
    );
    assert_eq!(bm2.browser, BrowserType::Brave);
    assert_eq!(bm2.domain_host(), "github.com");
}

#[test]
fn test_chromium_bookmarks_parser() {
    let json_content = r#"{
      "roots": {
        "bookmark_bar": {
          "children": [
            {
              "id": "1",
              "name": "Rust Docs",
              "type": "url",
              "url": "https://doc.rust-lang.org"
            },
            {
              "id": "2",
              "name": "Dev Folder",
              "type": "folder",
              "children": [
                {
                  "id": "3",
                  "name": "Crates.io",
                  "type": "url",
                  "url": "https://crates.io"
                }
              ]
            }
          ],
          "type": "folder",
          "name": "Bookmarks Bar"
        }
      }
    }"#;

    let bookmarks = ChromiumBookmarksParser::parse(json_content, BrowserType::Chrome);
    assert_eq!(bookmarks.len(), 2);
    assert_eq!(bookmarks[0].title, "Rust Docs");
    assert_eq!(bookmarks[0].folder, "Bookmarks Bar");
    assert_eq!(bookmarks[1].title, "Crates.io");
    assert_eq!(bookmarks[1].folder, "Bookmarks Bar/Dev Folder");
}

#[test]
fn test_netscape_html_parser_and_exporter() {
    let b1 = BookmarkFactory::create_chrome(
        "Rust Official",
        "https://www.rust-lang.org",
        "Programming",
    );
    let b2 = BookmarkFactory::create_firefox(
        "Mozilla MDN",
        "https://developer.mozilla.org",
        "Web Dev",
    );

    let list = vec![b1, b2];
    let html = NetscapeHtmlExporter::export("My Bookmarks Backup", &list);
    assert!(html.contains("<TITLE>My Bookmarks Backup</TITLE>"));
    assert!(html.contains("HREF=\"https://www.rust-lang.org\""));
    assert!(html.contains("<H3>Programming</H3>"));

    let re_parsed = NetscapeHtmlParser::parse(&html, BrowserType::Chrome);
    assert_eq!(re_parsed.len(), 2);
}

#[test]
fn test_analytics_and_markdown_export() {
    let b1 = BookmarkFactory::create_chrome("Site 1", "https://google.com/search", "General");
    let b2 = BookmarkFactory::create_chrome("Site 2", "https://google.com/maps", "General");
    let b3 = BookmarkFactory::create_brave("Site 3", "https://news.ycombinator.com", "News");

    let list = vec![b1, b2, b3];
    let stats = BookmarkAnalytics::compute_stats(&list);
    assert_eq!(stats.total_bookmarks, 3);
    assert_eq!(stats.total_folders, 2);
    assert_eq!(stats.top_domains[0].0, "google.com");
    assert_eq!(stats.top_domains[0].1, 2);

    let md = BookmarkMarkdownExporter::export("Bookmarks Summary", &list);
    assert!(md.contains("## 📁 General"));
    assert!(md.contains("- [Site 1](https://google.com/search)"));

    let json = BookmarkJsonExporter::export(&list).unwrap();
    let re_parsed = BookmarkJsonParser::parse(&json).unwrap();
    assert_eq!(re_parsed.len(), 3);
}
