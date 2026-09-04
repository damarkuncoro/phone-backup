use anyhow::Result;
use clap::{Args, Subcommand};
use std::fs;
use whatsapp::{
    ChatType, WhatsAppChatBuilder, WhatsAppCryptDecryptor, WhatsAppExportFactory,
    WhatsAppExportFormat, WhatsAppPathScanner, WhatsAppTxtParser,
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
    /// Decrypt WhatsApp crypt14/crypt15 database using a 64-digit hex key
    Decrypt {
        /// Encrypted database file path (e.g. msgstore.db.crypt14)
        file: String,
        /// 64-digit hex encryption key (or 32-byte key)
        #[arg(short, long)]
        key: String,
        /// Target decrypted SQLite output file path
        #[arg(short, long, default_value = "workspace/msgstore_decrypted.db")]
        output: String,
    },
    /// Export sample or indexed WhatsApp chat archive to standalone HTML / JSON
    Export {
        /// Optional exported chat text file (.txt from WhatsApp export)
        #[arg(short, long)]
        input: Option<String>,
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
        WhatsAppCommands::Decrypt { file, key, output } => {
            println!("Decrypting WhatsApp database: {}", file);
            let encrypted = fs::read(&file)?;
            let decrypted_db = WhatsAppCryptDecryptor::decrypt_with_hex_key(&encrypted, &key)?;
            fs::write(&output, &decrypted_db)?;
            println!("Successfully decrypted WhatsApp database ({} bytes) to: {}", decrypted_db.len(), output);
        }
        WhatsAppCommands::Export { input, format, output } => {
            let export_format = match format.to_lowercase().as_str() {
                "json" => WhatsAppExportFormat::Json,
                _ => WhatsAppExportFormat::Html,
            };

            let chats = if let Some(input_path) = input {
                let raw_text = fs::read_to_string(&input_path)?;
                let file_stem = std::path::Path::new(&input_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("WhatsApp Chat");
                let chat = WhatsAppTxtParser::parse(file_stem, &raw_text)?;
                println!("Parsed {} messages from: {}", chat.messages.len(), input_path);
                vec![chat]
            } else {
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
                vec![sample_chat]
            };

            let content = WhatsAppExportFactory::export(&chats, export_format)?;
            fs::write(&output, content)?;
            println!("WhatsApp archive exported successfully to: {}", output);
        }
    }
    Ok(())
}
