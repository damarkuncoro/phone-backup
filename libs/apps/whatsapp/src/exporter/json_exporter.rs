use crate::model::WhatsAppChat;
use anyhow::Result;

pub struct WhatsAppJsonExporter;

impl WhatsAppJsonExporter {
    pub fn export_pretty(chats: &[WhatsAppChat]) -> Result<String> {
        Ok(serde_json::to_string_pretty(chats)?)
    }
}
