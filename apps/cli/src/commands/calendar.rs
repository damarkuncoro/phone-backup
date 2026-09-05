use anyhow::Result;
use application::BackupService;
use calendar::{
    CalendarAnalytics, CalendarEvent, IcsExporter, IcsParser, JsonCalendarExporter,
};
use clap::Args;
use domain::DeviceId;
use std::path::Path;

#[derive(Args, Debug)]
pub struct CalendarArgs {
    /// Device ID, e.g. 10DDAJ0G7D0002L or A1B2C3D4
    pub id: String,

    /// Show only upcoming future events
    #[arg(short = 'u', long = "upcoming")]
    pub upcoming: bool,

    /// Filter by category (e.g. Work, Meeting, Birthday)
    #[arg(short = 'c', long = "category")]
    pub category: Option<String>,

    /// Search by event summary, description, or location
    #[arg(short = 'q', long = "query")]
    pub query: Option<String>,

    /// Maximum number of calendar events to display
    #[arg(short, long, default_value_t = 20)]
    pub limit: usize,

    /// Export calendar events to file (e.g. events.ics or events.json)
    #[arg(short, long)]
    pub export: Option<String>,
}

pub fn handle_calendar<D, S, R, T, A, DP, P>(
    args: CalendarArgs,
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
    println!("📅 Inspecting calendar schedule on device {}...", args.id);

    let all_files = service.scan_device(&device_id)?;
    let mut calendar_events: Vec<CalendarEvent> = Vec::new();

    // Discover .ics files on the device filesystem
    for file in all_files {
        let p = file.path.clone();
        let ext = Path::new(&p)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        if ext.eq_ignore_ascii_case("ics") || ext.eq_ignore_ascii_case("vcs") {
            if let Ok(bytes) = service.read_remote_header(&device_id, &p, 1048576) {
                if let Ok(ics_str) = String::from_utf8(bytes) {
                    let mut parsed = IcsParser::parse(&ics_str);
                    calendar_events.append(&mut parsed);
                }
            }
        }
    }

    let stats = CalendarAnalytics::compute_stats(&calendar_events);

    println!("\n📊 Calendar Schedule Summary:");
    println!("{:<22} {:>10}", "Total Events", stats.total_events);
    println!("{:<22} {:>10}", "Upcoming Events", stats.upcoming_count);
    println!("{:<22} {:>10}", "Past Events", stats.past_count);
    println!("{:<22} {:>10}", "All-Day Events", stats.all_day_count);
    println!("{:<22} {:>10}", "Recurring Events", stats.recurring_count);

    let conflicts = CalendarAnalytics::find_conflicts(&calendar_events);
    if !conflicts.is_empty() {
        println!("\n⚠️  Schedule Conflicts Detected ({})", conflicts.len());
        for (a, b) in conflicts.iter().take(3) {
            println!("   ⚡ \"{}\" overlaps with \"{}\"", a.summary, b.summary);
        }
    }

    let filtered_events = CalendarAnalytics::filter_events(
        calendar_events,
        args.upcoming,
        args.category.as_deref(),
        args.query.as_deref(),
    );

    println!("\n🗓️  Calendar Events ({})", filtered_events.len());
    println!("{:<32} {:<20} {:<18} LOCATION", "SUMMARY", "START TIME", "END TIME");
    println!("{}", "-".repeat(95));
    for event in filtered_events.iter().take(args.limit) {
        let loc = event.location.as_deref().unwrap_or("-");
        let start_str = if event.is_all_day {
            format!("All-Day ({})", event.start_time.format("%Y-%m-%d"))
        } else {
            event.start_time.format("%Y-%m-%d %H:%M").to_string()
        };
        let end_str = if event.is_all_day {
            "-".to_string()
        } else {
            event.end_time.format("%Y-%m-%d %H:%M").to_string()
        };

        println!(
            "{:<32} {:<20} {:<18} {}",
            event.summary, start_str, end_str, loc
        );

        if let Some(ref rrule) = event.recurrence {
            println!("   └── Recurrence: {}", rrule.format_description());
        }
    }

    if filtered_events.len() > args.limit {
        println!("... and {} more events (use --limit to show more)", filtered_events.len() - args.limit);
    }

    if let Some(ref target_file) = args.export {
        if target_file.ends_with(".json") {
            let json = JsonCalendarExporter::export_pretty(&filtered_events)?;
            std::fs::write(target_file, json)?;
            println!("💾 Exported {} calendar events to {}", filtered_events.len(), target_file);
        } else {
            let ics = IcsExporter::export(&filtered_events);
            std::fs::write(target_file, ics)?;
            println!("💾 Exported {} calendar events to {}", filtered_events.len(), target_file);
        }
    }

    Ok(())
}
