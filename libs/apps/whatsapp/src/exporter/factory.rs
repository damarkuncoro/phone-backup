use super::html_viewer::WhatsAppHtmlViewer;
use super::json_exporter::WhatsAppJsonExporter;
use crate::model::WhatsAppChat;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatsAppExportFormat {
    Html,
    Json,
}

pub struct WhatsAppExportFactory;

impl WhatsAppExportFactory {
    pub fn export(chats: &[WhatsAppChat], format: WhatsAppExportFormat) -> Result<String> {
        match format {
            WhatsAppExportFormat::Html => WhatsAppHtmlViewer::render_archive(chats),
            WhatsAppExportFormat::Json => WhatsAppJsonExporter::export_pretty(chats),
        }
    }
}
