use anyhow::Result;
use application::BackupService;
use clap::Args;
use domain::DeviceId;
use std::path::Path;
use wifi::{
    SecurityType, WifiAnalytics, WifiConfigStoreXmlParser, WifiCsvExporter, WifiJsonExporter,
    WifiQrGenerator, WpaSupplicantExporter, WpaSupplicantParser,
};

#[derive(Args, Debug)]
pub struct WifiArgs {
    /// Device ID, e.g. 10DDAJ0G7D0002L or A1B2C3D4
    pub id: String,

    /// Reveal Wi-Fi passwords in plain text instead of masking
    #[arg(short = 'p', long = "show-passwords")]
    pub show_passwords: bool,

    /// Generate Wi-Fi QR Code connection payload for specific SSID
    #[arg(short = 'q', long = "qr")]
    pub qr: Option<String>,

    /// Filter by security protocol (open, wpa2, wpa3, wep)
    #[arg(short = 's', long = "security")]
    pub security: Option<String>,

    /// Show only hidden networks
    #[arg(long = "hidden-only")]
    pub hidden_only: bool,

    /// Search networks by SSID keyword
    #[arg(long = "query")]
    pub query: Option<String>,

    /// Maximum number of networks to display
    #[arg(short, long, default_value_t = 25)]
    pub limit: usize,

    /// Import local WifiConfigStore.xml or wpa_supplicant.conf file
    #[arg(long)]
    pub import: Option<String>,

    /// Export saved Wi-Fi networks (.json, .csv, or .conf)
    #[arg(short, long)]
    pub export: Option<String>,
}

pub fn handle_wifi<D, S, R, T, A, DP, P>(
    args: WifiArgs,
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
    let mut networks = Vec::new();

    if let Some(ref import_path) = args.import {
        let p = Path::new(import_path);
        let content = std::fs::read_to_string(p)?;
        if p.extension().and_then(|e| e.to_str()) == Some("xml") || content.contains("<WifiConfig") {
            networks = WifiConfigStoreXmlParser::parse(&content);
        } else {
            networks = WpaSupplicantParser::parse(&content);
        }
        println!("📶 Loaded {} Wi-Fi networks from {}", networks.len(), import_path);
    } else {
        let device_id = DeviceId::new(&args.id);
        println!("📶 Scanning saved Wi-Fi networks on device {}...", args.id);

        let paths_to_probe = [
            "/data/misc/apexdata/com.android.wifi/WifiConfigStore.xml",
            "/data/misc/wifi/WifiConfigStore.xml",
            "/data/misc/wifi/wpa_supplicant.conf",
        ];

        for path in paths_to_probe {
            if let Ok(bytes) = service.read_remote_header(&device_id, path, 524288) {
                if let Ok(content) = String::from_utf8(bytes) {
                    if path.ends_with(".xml") {
                        let parsed = WifiConfigStoreXmlParser::parse(&content);
                        if !parsed.is_empty() {
                            networks.extend(parsed);
                            break;
                        }
                    } else {
                        let parsed = WpaSupplicantParser::parse(&content);
                        if !parsed.is_empty() {
                            networks.extend(parsed);
                            break;
                        }
                    }
                }
            }
        }
    }

    let stats = WifiAnalytics::compute_stats(&networks);

    println!("\n📊 Wi-Fi Configurations Summary:");
    println!("{:<24} {:>10}", "Total Networks", stats.total_networks);
    println!("{:<24} {:>10}", "Secured Networks", stats.secured_networks);
    println!("{:<24} {:>10}", "Open Networks", stats.open_networks);
    println!("{:<24} {:>10}", "Hidden Networks", stats.hidden_networks);
    println!("{:<24} {:>10}", "Metered Networks", stats.metered_networks);

    // If QR code is requested for specific SSID
    if let Some(ref qr_ssid) = args.qr {
        if let Some(target) = networks.iter().find(|n| n.ssid.eq_ignore_ascii_case(qr_ssid)) {
            println!("\n{}", WifiQrGenerator::render_terminal_card(target));
        } else {
            println!("\n⚠️ Network '{}' not found in backup.", qr_ssid);
        }
        return Ok(());
    }

    let sec_filter = args.security.as_deref().map(SecurityType::from_key_mgmt);
    let filtered = WifiAnalytics::filter_networks(
        networks,
        sec_filter,
        args.query.as_deref(),
        args.hidden_only,
    );

    println!("\n📋 Saved Wi-Fi Networks ({})", filtered.len());
    println!("{:<32} {:<12} {:<20} {:<8} METERED", "SSID", "SECURITY", "PASSWORD", "HIDDEN");
    println!("{}", "-".repeat(85));

    for net in filtered.iter().take(args.limit) {
        let pass_str = if args.show_passwords {
            net.pre_shared_key.as_deref().unwrap_or("[None]")
        } else {
            &net.masked_password()
        };

        println!(
            "{:<32} {:<12} {:<20} {:<8} {}",
            net.ssid,
            net.security_type.to_string(),
            pass_str,
            if net.is_hidden { "Yes" } else { "No" },
            if net.is_metered { "Yes" } else { "No" }
        );
    }

    if filtered.is_empty() {
        println!("   (No saved Wi-Fi networks found. On Android 10+, Wi-Fi credentials require Root or `--import WifiConfigStore.xml`)");
    } else if filtered.len() > args.limit {
        println!("... and {} more networks (use --limit to show more)", filtered.len() - args.limit);
    }

    if let Some(ref export_path) = args.export {
        let ext = Path::new(export_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("json");

        match ext {
            "csv" => {
                let csv = WifiCsvExporter::export(&filtered, args.show_passwords);
                std::fs::write(export_path, csv)?;
            }
            "conf" => {
                let conf = WpaSupplicantExporter::export(&filtered);
                std::fs::write(export_path, conf)?;
            }
            _ => {
                let json = WifiJsonExporter::export(&filtered)?;
                std::fs::write(export_path, json)?;
            }
        }
        println!("\n💾 Exported {} Wi-Fi networks to: {}", filtered.len(), export_path);
    }

    Ok(())
}
