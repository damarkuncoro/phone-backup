use anyhow::Result;
use application::BackupService;
use clap::Args;
use domain::DeviceId;
use notes::{
    KeepJsonParser, MarkdownNoteParser, NoteItem, NotesAnalytics, NotesHtmlExporter,
};
use std::path::Path;

#[derive(Args, Debug)]
pub struct NotesArgs {
    /// Device ID, e.g. 10DDAJ0G7D0002L or A1B2C3D4
    pub id: String,

    /// Filter by tag (e.g. work, todo, personal)
    #[arg(short = 't', long = "tag")]
    pub tag: Option<String>,

    /// Show only pinned notes
    #[arg(short = 'p', long = "pinned")]
    pub pinned: bool,

    /// Search by note title, body text, or checklist item
    #[arg(short = 'q', long = "query")]
    pub query: Option<String>,

    /// Maximum number of notes to display
    #[arg(short, long, default_value_t = 20)]
    pub limit: usize,

    /// Import Google Keep Takeout JSON file or directory
    #[arg(long)]
    pub import_keep: Option<String>,

    /// Export note collection to responsive offline HTML notes wall
    #[arg(long)]
    pub export_html: Option<String>,
}

pub fn handle_notes<D, S, R, T, A, DP, P>(
    args: NotesArgs,
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
    // Handle standalone Google Keep Takeout import
    if let Some(ref keep_path) = args.import_keep {
        let p = Path::new(keep_path);
        let mut keep_notes = Vec::new();

        if p.is_file() {
            let content = std::fs::read_to_string(p)?;
            if let Some(note) = KeepJsonParser::parse(&content) {
                keep_notes.push(note);
            }
        } else if p.is_dir() {
            for entry in std::fs::read_dir(p)?.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Some(note) = KeepJsonParser::parse(&content) {
                            keep_notes.push(note);
                        }
                    }
                }
            }
        }

        println!("📝 Imported {} Google Keep notes.", keep_notes.len());
        if let Some(ref html_path) = args.export_html {
            let html = NotesHtmlExporter::export("Google Keep Notes Archive", &keep_notes);
            std::fs::write(html_path, html)?;
            println!("🌐 Exported offline HTML notes wall to: {}", html_path);
        }
        return Ok(());
    }

    let device_id = DeviceId::new(&args.id);
    println!("📝 Scanning notes, memos, and checklists on device {}...", args.id);

    let all_files = service.scan_device(&device_id)?;
    let mut notes_collection: Vec<NoteItem> = Vec::new();

    // Discover markdown / notes files on the device filesystem
    for f in all_files {
        let p_lower = f.path.to_lowercase();
        let ext = Path::new(&f.path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if ext.eq_ignore_ascii_case("md") || (ext.eq_ignore_ascii_case("txt") && p_lower.contains("note")) {
            if let Ok(bytes) = service.read_remote_header(&device_id, &f.path, 131072) {
                if let Ok(text) = String::from_utf8(bytes) {
                    let mut note = MarkdownNoteParser::parse(&f.path, &text);
                    note.updated_at = f.modified_at;
                    notes_collection.push(note);
                }
            }
        }
    }

    let stats = NotesAnalytics::compute_stats(&notes_collection);

    println!("\n📊 Notes Collection Summary:");
    println!("{:<24} {:>10}", "Total Notes", stats.total_notes);
    println!("{:<24} {:>10}", "Checklists", stats.checklist_count);
    println!("{:<24} {:>10}", "Pinned Notes", stats.pinned_count);
    println!("{:<24} {:>10}", "Total Tasks", stats.total_tasks);
    println!("{:<24} {:>10}", "Completed Tasks", stats.completed_tasks);
    println!("{:<24} {:>10}", "Unique Tags", stats.tags_count.len());

    let filtered = NotesAnalytics::filter_notes(
        notes_collection,
        args.tag.as_deref(),
        args.pinned,
        args.query.as_deref(),
    );

    println!("\n📋 Discovered Notes ({})", filtered.len());
    println!("{:<32} {:<14} {:<16} PREVIEW", "TITLE", "TYPE", "TAGS");
    println!("{}", "-".repeat(95));
    for note in filtered.iter().take(args.limit) {
        let tags_str = if note.tags.is_empty() {
            "-".to_string()
        } else {
            note.tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" ")
        };
        let pin_prefix = if note.is_pinned { "📌 " } else { "" };
        let full_title = format!("{}{}", pin_prefix, note.title);

        println!(
            "{:<32} {:<14} {:<16} {}",
            full_title,
            note.note_type.to_string(),
            tags_str,
            note.snippet(32)
        );
    }

    if filtered.len() > args.limit {
        println!("... and {} more notes (use --limit to show more)", filtered.len() - args.limit);
    }

    if let Some(ref html_path) = args.export_html {
        let html = NotesHtmlExporter::export("Device Notes Backup", &filtered);
        std::fs::write(html_path, html)?;
        println!("🌐 Exported offline HTML notes wall to: {}", html_path);
    }

    Ok(())
}
