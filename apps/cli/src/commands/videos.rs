use anyhow::Result;
use application::BackupService;
use clap::Args;
use domain::DeviceId;
use std::collections::BTreeMap;
use std::path::Path;
use video_engine::{VideoContainer, VideoItem, VideoQuality};

#[derive(Args, Debug)]
pub struct VideosArgs {
    /// Device ID, e.g. 10DDAJ0G7D0002L or A1B2C3D4
    pub id: String,

    /// Filter by quality tier (4k, 2k, 1080p, 720p, 480p)
    #[arg(short = 'q', long = "quality")]
    pub quality: Option<String>,

    /// Filter by container format (mp4, mkv, webm, avi, mov, 3gp)
    #[arg(short = 'c', long = "container")]
    pub container: Option<String>,

    /// Filter by minimum file size in bytes
    #[arg(long)]
    pub min_size: Option<u64>,

    /// Maximum number of videos to display
    #[arg(short, long, default_value_t = 20)]
    pub limit: usize,

    /// Display extracted metadata preview (resolution, codec, bitrate)
    #[arg(short, long)]
    pub preview: bool,
}

fn format_bytes(bytes: u64) -> String {
    match number_prefix::NumberPrefix::decimal(bytes as f64) {
        number_prefix::NumberPrefix::Standalone(b) => format!("{:.0} B", b),
        number_prefix::NumberPrefix::Prefixed(p, n) => format!("{:.2} {}B", n, p),
    }
}

pub fn handle_videos<D, S, R, T, A, DP, P>(
    args: VideosArgs,
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
    let device_id = DeviceId::new(&args.id);
    println!("🎬 Inspecting video gallery on device {}...", args.id);

    let all_files = service.scan_device(&device_id)?;
    let mut video_items: Vec<VideoItem> = all_files
        .into_iter()
        .filter_map(|f| {
            let path_str = f.path.clone();
            let ext = Path::new(&path_str)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            let container = VideoContainer::from_extension(ext);
            if matches!(container, VideoContainer::Other(_)) {
                return None;
            }

            let mut item = VideoItem::new(path_str, f.size_bytes, container);
            item = item.with_modified(f.modified_at);
            Some(item)
        })
        .collect();

    let mut container_counts = BTreeMap::new();
    let mut container_bytes = BTreeMap::new();
    for v in &video_items {
        *container_counts.entry(v.container.to_string()).or_insert(0) += 1;
        *container_bytes.entry(v.container.to_string()).or_insert(0) += v.size_bytes;
    }

    println!("\n📊 Video Library Summary:");
    println!("{:<20} {:>10} {:>15}", "CONTAINER", "FILES", "VOLUME");
    println!("{}", "-".repeat(50));
    for (name, count) in &container_counts {
        let bytes = container_bytes.get(name).copied().unwrap_or(0);
        println!("{:<20} {:>10} {:>15}", name, count, format_bytes(bytes));
    }

    let parsed_container = args.container.as_deref().map(VideoContainer::from_extension);
    let parsed_quality = args.quality.as_deref().map(|q| match q.to_lowercase().as_str() {
        "4k" | "uhd" => VideoQuality::Uhd4K,
        "2k" | "qhd" => VideoQuality::Qhd2K,
        "1080p" | "fhd" => VideoQuality::Fhd1080p,
        "720p" | "hd" => VideoQuality::Hd720p,
        "480p" | "sd" => VideoQuality::Sd480p,
        _ => VideoQuality::Unknown,
    });

    video_items.retain(|v| {
        if let Some(ref c) = parsed_container {
            if &v.container != c {
                return false;
            }
        }
        if let Some(min) = args.min_size {
            if v.size_bytes < min {
                return false;
            }
        }
        if let Some(ref q) = parsed_quality {
            if v.quality() != *q {
                return false;
            }
        }
        true
    });

    video_items.sort_by_key(|b| std::cmp::Reverse(b.modified_at));

    println!("\n🎥 Discovered Videos ({})", video_items.len());
    println!("{:<45} {:<15} {:>12}  MODIFIED", "PATH", "CONTAINER", "SIZE");
    println!("{}", "-".repeat(95));
    for item in video_items.iter_mut().take(args.limit) {
        if args.preview && item.metadata.is_none() {
            if let Ok(bytes) = service.read_remote_header(&device_id, &item.path, 131072) {
                *item = video_engine::VideoAnalyzer::enrich_item(item.clone(), &bytes);
            }
        }

        let name = Path::new(&item.path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&item.path);

        let mod_str = item
            .modified_at
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{:<45} {:<15} {:>12}  {}",
            name,
            item.container.to_string(),
            format_bytes(item.size_bytes),
            mod_str
        );

        if args.preview {
            if let Some(ref meta) = item.metadata {
                println!(
                    "   ↳ Quality: {} | Res: {} | Dur: {} | Codec: {}",
                    meta.quality_tier,
                    meta.format_resolution(),
                    meta.format_duration(),
                    meta.video_codec.as_deref().unwrap_or("Unknown")
                );
            }
        }
    }

    if video_items.len() > args.limit {
        println!("... and {} more videos (use --limit to show more)", video_items.len() - args.limit);
    }

    Ok(())
}
