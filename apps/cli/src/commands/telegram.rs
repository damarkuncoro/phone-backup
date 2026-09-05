use anyhow::Result;
use application::BackupService;
use clap::Args;
use domain::DeviceId;
use std::collections::BTreeMap;
use std::path::Path;
use telegram::{TelegramMediaType, TelegramPathResolver};

#[derive(Args, Debug)]
pub struct TelegramArgs {
    /// Device ID, e.g. 10DDAJ0G7D0002L or A1B2C3D4
    pub id: String,

    /// Filter by media type (voice, video-note, photo, video, document)
    #[arg(short = 't', long = "type")]
    pub media_type: Option<String>,

    /// Filter by minimum file size in bytes
    #[arg(long)]
    pub min_size: Option<u64>,

    /// Maximum number of discovered items to display
    #[arg(short, long, default_value_t = 20)]
    pub limit: usize,

    /// Import Telegram Desktop export (result.json) and render offline HTML archive
    #[arg(long)]
    pub import_json: Option<String>,

    /// Output path for HTML chat archive
    #[arg(long)]
    pub export_html: Option<String>,
}

fn format_bytes(bytes: u64) -> String {
    match number_prefix::NumberPrefix::decimal(bytes as f64) {
        number_prefix::NumberPrefix::Standalone(b) => format!("{:.0} B", b),
        number_prefix::NumberPrefix::Prefixed(p, n) => format!("{:.2} {}B", n, p),
    }
}

pub fn handle_telegram<D, S, R, T, A, DP, P>(
    args: TelegramArgs,
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
    // Handle standalone Desktop JSON export rendering if requested
    if let Some(ref json_path) = args.import_json {
        let json_str = std::fs::read_to_string(json_path)?;
        if let Some(chat) = telegram::TelegramJsonParser::parse(&json_str) {
            println!("💬 Parsed Telegram Chat: \"{}\" ({} messages)", chat.title, chat.total_messages());
            if let Some(ref html_path) = args.export_html {
                let html = telegram::TelegramHtmlExporter::export(&chat);
                std::fs::write(html_path, html)?;
                println!("🌐 Exported offline HTML chat viewer to: {}", html_path);
            }
            return Ok(());
        }
    }

    let device_id = DeviceId::new(&args.id);
    println!("✈️  Inspecting Telegram data and media on device {}...", args.id);

    let all_files = service.scan_device(&device_id)?;
    let mut telegram_files: Vec<(String, u64, chrono::DateTime<chrono::Utc>, TelegramMediaType)> = Vec::new();

    for f in all_files {
        let p_lower = f.path.to_lowercase();
        if p_lower.contains("org.telegram.messenger") || p_lower.contains("/telegram/") || p_lower.contains("/telegram ") {
            let mt = TelegramPathResolver::classify_path(&f.path);
            telegram_files.push((f.path, f.size_bytes, f.modified_at, mt));
        }
    }

    let mut type_counts = BTreeMap::new();
    let mut type_bytes = BTreeMap::new();
    for (_, size, _, mt) in &telegram_files {
        *type_counts.entry(mt.to_string()).or_insert(0) += 1;
        *type_bytes.entry(mt.to_string()).or_insert(0) += *size;
    }

    println!("\n📊 Telegram Media Storage Summary:");
    println!("{:<24} {:>10} {:>15}", "MEDIA CATEGORY", "FILES", "VOLUME");
    println!("{}", "-".repeat(52));
    for (name, count) in &type_counts {
        let bytes = type_bytes.get(name).copied().unwrap_or(0);
        println!("{:<24} {:>10} {:>15}", name, count, format_bytes(bytes));
    }

    let parsed_type = args.media_type.as_deref().map(|t| match t.to_lowercase().as_str() {
        "voice" | "ptt" | "audio" => TelegramMediaType::VoiceNote,
        "video-note" | "round" => TelegramMediaType::VideoNote,
        "photo" | "image" | "images" => TelegramMediaType::Photo,
        "video" | "videos" => TelegramMediaType::Video,
        "doc" | "document" | "documents" => TelegramMediaType::Document,
        _ => TelegramMediaType::Unknown,
    });

    telegram_files.retain(|(_, size, _, mt)| {
        if let Some(ref target_mt) = parsed_type {
            if mt != target_mt {
                return false;
            }
        }
        if let Some(min) = args.min_size {
            if *size < min {
                return false;
            }
        }
        true
    });

    telegram_files.sort_by(|a, b| b.2.cmp(&a.2));

    println!("\n📂 Discovered Telegram Media ({})", telegram_files.len());
    println!("{:<45} {:<20} {:>12}  MODIFIED", "FILENAME", "TYPE", "SIZE");
    println!("{}", "-".repeat(95));
    for (path, size, modified, mt) in telegram_files.iter().take(args.limit) {
        let name = Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(path);

        let mod_str = modified.format("%Y-%m-%d %H:%M:%S").to_string();

        println!(
            "{:<45} {:<20} {:>12}  {}",
            name,
            mt.to_string(),
            format_bytes(*size),
            mod_str
        );
    }

    if telegram_files.len() > args.limit {
        println!("... and {} more items (use --limit to show more)", telegram_files.len() - args.limit);
    }

    Ok(())
}
