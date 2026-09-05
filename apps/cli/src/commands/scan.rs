use anyhow::Result;
use application::BackupService;
use domain::{DeviceId, FileEntry, ScanCategory, ScanCategorySummary, ScanFilter, ScanResult};
use scanner_engine::FileClassifier;
use std::collections::BTreeMap;

fn format_bytes(bytes: u64) -> String {
    match number_prefix::NumberPrefix::decimal(bytes as f64) {
        number_prefix::NumberPrefix::Standalone(b) => format!("{:.0} B", b),
        number_prefix::NumberPrefix::Prefixed(p, n) => format!("{:.2} {}B", n, p),
    }
}

fn render_storage_bar(categories: &BTreeMap<ScanCategory, ScanCategorySummary>, total_bytes: u64) {
    if total_bytes == 0 {
        return;
    }

    let bar_width = 40;
    let mut bar = String::new();
    let symbols = [
        (ScanCategory::Videos, "█"),
        (ScanCategory::Photos, "▓"),
        (ScanCategory::WhatsApp, "▒"),
        (ScanCategory::Audio, "░"),
        (ScanCategory::Documents, "▰"),
        (ScanCategory::Apks, "▱"),
        (ScanCategory::Other, "·"),
    ];

    let mut legend = Vec::new();
    let mut allocated_chars = 0;

    for (cat, sym) in symbols {
        if let Some(sum) = categories.get(&cat) {
            if sum.total_bytes > 0 {
                let pct = (sum.total_bytes as f64 / total_bytes as f64) * 100.0;
                let chars = ((pct / 100.0) * bar_width as f64).round() as usize;
                let count = chars.min(bar_width - allocated_chars);
                bar.push_str(&sym.repeat(count));
                allocated_chars += count;
                legend.push(format!("{} {} ({:.1}%)", sym, cat, pct));
            }
        }
    }

    if allocated_chars < bar_width {
        bar.push_str(&"·".repeat(bar_width - allocated_chars));
    }

    println!("\n📦 Storage Breakdown [{}]:", format_bytes(total_bytes));
    println!("[{}]", bar);
    println!("{}", legend.join("  "));
}

pub fn handle_scan<D, S, R, T, A, DP, P>(
    service: &BackupService<D, S, R, T, A, DP, P>,
    id: &str,
    category: Option<String>,
    min_size: Option<u64>,
    max_size: Option<u64>,
    sort: &str,
    limit: usize,
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
    let device_id = DeviceId::new(id);
    println!("🔍 Scanning device {} with Specialist Engine V5...", id);

    let filter = ScanFilter {
        min_size_bytes: min_size,
        max_size_bytes: max_size,
        ..Default::default()
    };

    let res = match service.scan_device_detailed(&device_id, vec![], Some(&filter)) {
        Ok(r) => r,
        Err(_) => {
            let files = service.scan_device(&device_id)?;
            ScanResult::new(files, Vec::new())
        }
    };

    let total_bytes = res.total_bytes();

    println!("\n📊 Category Summary:");
    println!("{:<15} {:>10} {:>15}", "CATEGORY", "FILES", "VOLUME");
    println!("{}", "-".repeat(45));
    for (cat, sum) in &res.categories {
        println!("{:<15} {:>10} {:>15}", cat.to_string(), sum.file_count, format_bytes(sum.total_bytes));
    }

    render_storage_bar(&res.categories, total_bytes);

    if let Some(m) = &res.metrics {
        println!("\n⚡ Performance: {} files in {}ms ({:.0} files/sec)", m.files_scanned, m.duration_ms, m.throughput_files_per_sec);
    }

    let mut filtered_files: Vec<FileEntry> = if let Some(cat_filter) = category {
        let target = cat_filter.to_lowercase();
        res.files
            .into_iter()
            .filter(|f| FileClassifier::classify(f).to_string().to_lowercase() == target)
            .collect()
    } else {
        res.files
    };

    match sort {
        "size" => filtered_files.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes)),
        "date" => filtered_files.sort_by(|a, b| b.modified_at.cmp(&a.modified_at)),
        _ => filtered_files.sort_by(|a, b| a.path.cmp(&b.path)),
    }

    println!("\n📁 Discovered Files ({})", filtered_files.len());
    println!("{:<50} {:>12}  MODIFIED", "PATH", "SIZE");
    println!("{}", "-".repeat(85));
    for f in filtered_files.iter().take(limit) {
        println!("{:<50} {:>12}  {}", f.path, format_bytes(f.size_bytes), f.modified_at.format("%Y-%m-%d %H:%M:%S"));
    }
    if filtered_files.len() > limit {
        println!("... and {} more files (use --limit to show more)", filtered_files.len() - limit);
    }

    Ok(())
}
