use anyhow::Result;
use application::BackupService;
use clap::{Args, Subcommand};
use domain::SnapshotId;
use ports::{
    AppProviderPort, DataProviderPort, DevicePort, ProgressPort, RepositoryPort, ScannerPort,
    StoragePort,
};
use std::fs;

#[derive(Args, Debug)]
pub struct ExportArgs {
    #[command(subcommand)]
    pub command: ExportCommands,
}

#[derive(Subcommand, Debug)]
pub enum ExportCommands {
    /// Export contacts to vCard or CSV
    Contacts {
        /// Snapshot ID
        snapshot: String,
        /// Export format: vcard, csv
        #[arg(short, long, default_value = "vcard")]
        format: String,
        /// Optional output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Export SMS messages to XML, HTML, CSV, or JSON
    Sms {
        /// Snapshot ID
        snapshot: String,
        /// Export format: xml, html, csv, json
        #[arg(short, long, default_value = "xml")]
        format: String,
        /// Optional output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Export call logs to JSON or compute statistics
    Calls {
        /// Snapshot ID
        snapshot: String,
        /// Export format: json, stats
        #[arg(short, long, default_value = "json")]
        format: String,
        /// Optional output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

pub fn handle_export<D, S, R, T, A, DP, P>(
    args: ExportArgs,
    service: &BackupService<D, S, R, T, A, DP, P>,
) -> Result<()>
where
    D: DevicePort,
    S: ScannerPort,
    R: RepositoryPort,
    T: StoragePort,
    A: AppProviderPort,
    DP: DataProviderPort,
    P: ProgressPort,
{
    match args.command {
        ExportCommands::Contacts { snapshot, format, output } => {
            let snap_id = SnapshotId(snapshot);
            let content = match format.to_lowercase().as_str() {
                "csv" => service.export_contacts_csv(&snap_id)?,
                _ => service.export_contacts_vcard(&snap_id)?,
            };
            write_or_print(content, output, "Contacts exported successfully")?;
        }
        ExportCommands::Sms { snapshot, format, output } => {
            let snap_id = SnapshotId(snapshot);
            let content = match format.to_lowercase().as_str() {
                "html" => service.export_sms_html(&snap_id)?,
                "json" => service.export_sms_json(&snap_id)?,
                _ => service.export_sms_xml(&snap_id)?,
            };
            write_or_print(content, output, "SMS messages exported successfully")?;
        }
        ExportCommands::Calls { snapshot, format, output } => {
            let snap_id = SnapshotId(snapshot);
            if format.to_lowercase() == "stats" {
                let stats = service.get_call_stats(&snap_id)?;
                let json_stats = serde_json::to_string_pretty(&stats)?;
                write_or_print(json_stats, output, "Call log statistics computed successfully")?;
            } else {
                let content = service.export_call_logs_json(&snap_id)?;
                write_or_print(content, output, "Call logs exported successfully")?;
            }
        }
    }
    Ok(())
}

fn write_or_print(content: String, output: Option<String>, success_msg: &str) -> Result<()> {
    if let Some(path) = output {
        fs::write(&path, content)?;
        println!("{} to: {}", success_msg, path);
    } else {
        println!("{}", content);
    }
    Ok(())
}
