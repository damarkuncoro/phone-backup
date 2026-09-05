use anyhow::Result;
use application::BackupService;
use bookmarks::{
    BookmarkAnalytics, BookmarkItem, BookmarkJsonExporter, BookmarkMarkdownExporter, BrowserType,
    ChromiumBookmarksParser, NetscapeHtmlExporter, NetscapeHtmlParser,
};
use clap::Args;
use domain::DeviceId;
use std::path::Path;

#[derive(Args, Debug)]
pub struct BookmarksArgs {
    /// Device ID, e.g. 10DDAJ0G7D0002L or A1B2C3D4
    pub id: String,

    /// Filter by browser (chrome, brave, edge, firefox, samsung)
    #[arg(short = 'b', long = "browser")]
    pub browser: Option<String>,

    /// Filter by folder name
    #[arg(short = 'f', long = "folder")]
    pub folder: Option<String>,

    /// Search bookmarks by title or URL keyword
    #[arg(short = 'q', long = "query")]
    pub query: Option<String>,

    /// Maximum number of bookmarks to display
    #[arg(short, long, default_value_t = 30)]
    pub limit: usize,

    /// Import local Chrome Bookmarks JSON or Netscape HTML file
    #[arg(long)]
    pub import: Option<String>,

    /// Export bookmarks to universal Netscape HTML file (importable in any browser)
    #[arg(long)]
    pub export_html: Option<String>,

    /// Export bookmarks to categorized Markdown document
    #[arg(long)]
    pub export_md: Option<String>,

    /// Export bookmarks to JSON file
    #[arg(long)]
    pub export_json: Option<String>,
}

pub fn handle_bookmarks<D, S, R, T, A, DP, P>(
    args: BookmarksArgs,
    service: &BackupService<D, S, R, T, A, DP, P>,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    let mut bookmarks: Vec<BookmarkItem> = Vec::new();

    if let Some(ref import_path) = args.import {
        let p = Path::new(import_path);
        let content = std::fs::read_to_string(p)?;
        let b_type = args
            .browser
            .as_deref()
            .map(BrowserType::from_package_or_path)
            .unwrap_or(BrowserType::Chrome);

        if content.contains("<!DOCTYPE NETSCAPE") || content.contains("<H3>") {
            bookmarks = NetscapeHtmlParser::parse(&content, b_type);
        } else {
            bookmarks = ChromiumBookmarksParser::parse(&content, b_type);
        }
        println!("🔖 Imported {} bookmarks from {}", bookmarks.len(), import_path);
    } else {
        let device_id = DeviceId::new(&args.id);
        println!("🔖 Scanning browser bookmarks on device {}...", args.id);

        let chrome_paths = [
            (
                "/data/data/com.android.chrome/app_chrome/Default/Bookmarks",
                BrowserType::Chrome,
            ),
            (
                "/data/data/com.brave.browser/app_brave/Default/Bookmarks",
                BrowserType::Brave,
            ),
            (
                "/data/data/com.microsoft.emmx/app_edge/Default/Bookmarks",
                BrowserType::Edge,
            ),
        ];

        for (path, b_type) in chrome_paths {
            if let Ok(bytes) = service.read_remote_header(&device_id, path, 1048576) {
                if let Ok(content) = String::from_utf8(bytes) {
                    let parsed = ChromiumBookmarksParser::parse(&content, b_type);
                    if !parsed.is_empty() {
                        bookmarks.extend(parsed);
                    }
                }
            }
        }
    }

    let stats = BookmarkAnalytics::compute_stats(&bookmarks);

    println!("\n📊 Bookmarks Collection Summary:");
    println!("{:<24} {:>10}", "Total Bookmarks", stats.total_bookmarks);
    println!("{:<24} {:>10}", "Folders", stats.total_folders);
    println!("{:<24} {:>10}", "Distinct Domains", stats.top_domains.len());

    if !stats.top_domains.is_empty() {
        println!("\n🌐 Top Domains:");
        for (domain, count) in stats.top_domains.iter().take(5) {
            println!("  • {:<30} ({} bookmarks)", domain, count);
        }
    }

    let browser_filter = args
        .browser
        .as_deref()
        .map(BrowserType::from_package_or_path);

    let filtered = BookmarkAnalytics::filter_bookmarks(
        bookmarks,
        browser_filter,
        args.folder.as_deref(),
        args.query.as_deref(),
    );

    println!("\n📋 Bookmarks List ({})", filtered.len());
    println!("{:<32} {:<24} {:<20} URL", "TITLE", "DOMAIN", "FOLDER");
    println!("{}", "-".repeat(105));

    for bm in filtered.iter().take(args.limit) {
        let clean_title = if bm.title.chars().count() > 30 {
            let truncated: String = bm.title.chars().take(27).collect();
            format!("{}...", truncated)
        } else {
            bm.title.clone()
        };

        println!(
            "{:<32} {:<24} {:<20} {}",
            clean_title,
            bm.domain_host(),
            bm.folder,
            bm.url
        );
    }

    if filtered.is_empty() {
        println!("   (No bookmarks found on storage. Use `--import <PATH>` to import Chrome Bookmarks JSON or HTML files)");
    } else if filtered.len() > args.limit {
        println!("... and {} more bookmarks (use --limit to show more)", filtered.len() - args.limit);
    }

    if let Some(ref html_path) = args.export_html {
        let html = NetscapeHtmlExporter::export("Device Bookmarks Backup", &filtered);
        std::fs::write(html_path, html)?;
        println!("\n🌐 Exported universal Netscape HTML bookmarks to: {}", html_path);
    }

    if let Some(ref md_path) = args.export_md {
        let md = BookmarkMarkdownExporter::export("Bookmarks Archive", &filtered);
        std::fs::write(md_path, md)?;
        println!("\n📝 Exported Markdown bookmarks to: {}", md_path);
    }

    if let Some(ref json_path) = args.export_json {
        let json = BookmarkJsonExporter::export(&filtered)?;
        std::fs::write(json_path, json)?;
        println!("\n💾 Exported JSON bookmarks to: {}", json_path);
    }

    Ok(())
}
