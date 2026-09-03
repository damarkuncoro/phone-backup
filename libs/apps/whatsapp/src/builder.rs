use crate::exporter::{WhatsAppExportFactory, WhatsAppExportFormat};
use crate::indexer::{WhatsAppMediaIndexer, WhatsAppMediaSummary};
use crate::model::{ChatType, WhatsAppChat, WhatsAppMediaItem, WhatsAppMessage};
use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct WhatsAppChatBuilder {
    chat: WhatsAppChat,
}

impl WhatsAppChatBuilder {
    pub fn new(jid: impl Into<String>, chat_type: ChatType) -> Self {
        Self {
            chat: WhatsAppChat::new(jid, None, chat_type),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.chat.name = Some(name.into());
        self
    }

    pub fn add_text_message(mut self, id: impl Into<String>, sender_jid: impl Into<String>, from_me: bool, timestamp: DateTime<Utc>, body: impl Into<String>) -> Self {
        let msg = WhatsAppMessage::new_text(id, self.chat.jid.clone(), sender_jid, from_me, timestamp, body);
        self.chat.add_message(msg);
        self
    }

    pub fn add_message(mut self, msg: WhatsAppMessage) -> Self {
        self.chat.add_message(msg);
        self
    }

    pub fn build(self) -> WhatsAppChat {
        self.chat
    }
}

pub struct WhatsAppBackupStore {
    pub chats: Vec<WhatsAppChat>,
    pub media_items: Vec<WhatsAppMediaItem>,
}

impl WhatsAppBackupStore {
    pub fn new() -> Self {
        Self {
            chats: Vec::new(),
            media_items: Vec::new(),
        }
    }

    pub fn add_chat(&mut self, chat: WhatsAppChat) {
        self.chats.push(chat);
    }

    pub fn index_media_file(&mut self, rel_path: &str, size_bytes: u64) {
        let item = WhatsAppMediaIndexer::parse_media_item(rel_path, size_bytes);
        self.media_items.push(item);
    }

    pub fn media_summary(&self) -> WhatsAppMediaSummary {
        WhatsAppMediaIndexer::summarize(&self.media_items)
    }

    pub fn export_chats(&self, format: WhatsAppExportFormat) -> Result<String> {
        WhatsAppExportFactory::export(&self.chats, format)
    }
}

impl Default for WhatsAppBackupStore {
    fn default() -> Self {
        Self::new()
    }
}
