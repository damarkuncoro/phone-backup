use anyhow::Result;
use clap::{Args, Subcommand};
use std::fs;
use whatsapp::{
    ChatType, WhatsAppBackupStore, WhatsAppChatBuilder, WhatsAppExportFactory,
    WhatsAppExportFormat, WhatsAppPathScanner,
};

#[derive(Args, Debug)]
pub struct WhatsAppArgs {
    #[command(subcommand)]
    pub command: WhatsAppCommands,
}

#[derive(Subcommand, Debug)]
pub enum WhatsAppCommands {
    /// List known WhatsApp storage locations across Android versions
    Paths,
    /// Export sample or indexed WhatsApp chat archive to standalone HTML / JSON
    Export {
        /// Format: html, json
        #[arg(short, long, default_value = "html")]
        format: String,
        /// Target output file path
        #[arg(short, long, default_value = "whatsapp_archive.html")]
        output: String,
    },
}

pub fn handle_whatsapp(args: WhatsAppArgs) -> Result<()> {
    match args.command {
        WhatsAppCommands::Paths => {
            println!("WhatsApp Storage Locations:");
            println!("---------------------------");
            for (idx, path) in WhatsAppPathScanner::candidate_roots().iter().enumerate() {
                println!("{}. {}", idx + 1, path);
            }
        }
        WhatsAppCommands::Export { format, output } => {
            let mut store = WhatsAppBackupStore::new();
            let sample_chat = WhatsAppChatBuilder::new("sample_chat@s.whatsapp.net", ChatType::Individual)
                .with_name("WhatsApp Backup Archive")
                .add_text_message(
                    "msg1",
                    "sender",
                    false,
                    chrono::Utc::now(),
                    "Welcome to your offline WhatsApp Backup Archive!",
                )
                .build();
            store.add_chat(sample_chat);

            let export_format = match format.to_lowercase().as_str() {
                "json" => WhatsAppExportFormat::Json,
                _ => WhatsAppExportFormat::Html,
            };

            let content = WhatsAppExportFactory::export(&store.chats, export_format)?;
            fs::write(&output, content)?;
            println!("WhatsApp archive exported successfully to: {}", output);
        }
    }
    Ok(())
}
