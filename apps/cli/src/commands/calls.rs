use anyhow::Result;
use application::BackupService;
use calls::{CallAnalytics, CallLogItem, CallType, CsvCallExporter, JsonCallExporter, XmlCallParser};
use clap::Args;
use domain::DeviceId;

#[derive(Args, Debug)]
pub struct CallsArgs {
    /// Device ID, e.g. 10DDAJ0G7D0002L or A1B2C3D4
    pub id: String,

    /// Filter by call type (incoming, outgoing, missed, rejected)
    #[arg(short = 't', long = "type")]
    pub call_type: Option<String>,

    /// Search by contact name or phone number
    #[arg(short = 'c', long = "contact")]
    pub contact: Option<String>,

    /// Filter by minimum duration in seconds
    #[arg(long)]
    pub min_duration: Option<u64>,

    /// Maximum number of call logs to display
    #[arg(short, long, default_value_t = 20)]
    pub limit: usize,

    /// Export call history to a local file (e.g. calls.csv or calls.json)
    #[arg(short, long)]
    pub export: Option<String>,
}

pub fn handle_calls<D, S, R, T, A, DP, P>(
    args: CallsArgs,
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
    println!("📞 Fetching call history from device {}...", args.id);

    let raw_logs = service.list_call_logs(&device_id).unwrap_or_default();
    let mut call_items: Vec<CallLogItem> = if !raw_logs.is_empty() {
        raw_logs
            .into_iter()
            .enumerate()
            .map(|(idx, r)| {
                let ct = CallType::from_android_type(r.type_code as u32);
                let mut item = CallLogItem::new(
                    format!("call_{}", idx + 1),
                    r.number,
                    ct,
                    r.date,
                    r.duration_seconds as u64,
                );
                if let Some(name) = r.name {
                    item = item.with_name(name);
                }
                item
            })
            .collect()
    } else {
        // Check for common XML call log backup paths on device
        let sample_xml = service
            .read_remote_header(&device_id, "/sdcard/calls.xml", 524288)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok());
        if let Some(ref xml) = sample_xml {
            XmlCallParser::parse(xml)
        } else {
            Vec::new()
        }
    };

    let stats = CallAnalytics::compute_stats(&call_items);

    println!("\n📊 Call History Summary:");
    println!("{:<22} {:>10}", "Total Calls", stats.total_calls);
    println!("{:<22} {:>10}", "Total Talk Time", stats.format_total_duration());
    println!("{:<22} {:>10}", "Incoming Calls", stats.incoming_count);
    println!("{:<22} {:>10}", "Outgoing Calls", stats.outgoing_count);
    println!("{:<22} {:>10}", "Missed Calls", stats.missed_count);
    println!("{:<22} {:>10}", "Rejected Calls", stats.rejected_count);
    println!("{:<22} {:>9.1}%", "Missed Call Rate", stats.missed_percentage());

    if !stats.frequent_contacts.is_empty() {
        println!("\n⭐ Top Frequent Callers:");
        println!("{:<30} {:<18} {:>8} {:>12}", "NAME", "NUMBER", "CALLS", "TALK TIME");
        println!("{}", "-".repeat(72));
        for contact in stats.frequent_contacts.iter().take(5) {
            let name = contact.contact_name.as_deref().unwrap_or("-");
            let mins = contact.total_duration_secs / 60;
            let secs = contact.total_duration_secs % 60;
            let dur_str = format!("{}m {}s", mins, secs);
            println!(
                "{:<30} {:<18} {:>8} {:>12}",
                name, contact.phone_number, contact.call_count, dur_str
            );
        }
    }

    let parsed_type = args.call_type.as_deref().map(CallType::from_str_loose);
    call_items = CallAnalytics::filter_calls(
        call_items,
        parsed_type,
        args.contact.as_deref(),
        args.min_duration,
    );
    call_items.sort_by_key(|b| std::cmp::Reverse(b.timestamp));

    println!("\n📋 Call Records ({})", call_items.len());
    println!("{:<28} {:<18} {:<16} {:>10}  TIMESTAMP", "NAME", "NUMBER", "TYPE", "DURATION");
    println!("{}", "-".repeat(95));
    for item in call_items.iter().take(args.limit) {
        let name = item.contact_name.as_deref().unwrap_or("-");
        println!(
            "{:<28} {:<18} {:<16} {:>10}  {}",
            name,
            item.phone_number,
            item.call_type.to_string(),
            item.duration_display(),
            item.timestamp.format("%Y-%m-%d %H:%M:%S")
        );
    }

    if call_items.len() > args.limit {
        println!("... and {} more call records (use --limit to show more)", call_items.len() - args.limit);
    }

    if let Some(ref target_file) = args.export {
        if target_file.ends_with(".json") {
            let json = JsonCallExporter::export_pretty(&call_items)?;
            std::fs::write(target_file, json)?;
            println!("💾 Exported {} call records to {}", call_items.len(), target_file);
        } else {
            let csv = CsvCallExporter::export(&call_items);
            std::fs::write(target_file, csv)?;
            println!("💾 Exported {} call records to {}", call_items.len(), target_file);
        }
    }

    Ok(())
}
