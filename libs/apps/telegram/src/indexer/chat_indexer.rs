use crate::domain::{TelegramChat, TelegramMediaType, TelegramMessage};
use std::collections::HashMap;

/// Domain service for indexing and analyzing Telegram chat histories.
pub struct TelegramChatIndexer;

impl TelegramChatIndexer {
    /// Computes distribution of media attachments across chat messages.
    pub fn compute_media_stats(chat: &TelegramChat) -> HashMap<TelegramMediaType, usize> {
        let mut counts = HashMap::new();
        for msg in &chat.messages {
            *counts.entry(msg.media_type).or_insert(0) += 1;
        }
        counts
    }

    /// Filters chat messages by media type, sender, or text query.
    pub fn filter_messages(
        chat: &TelegramChat,
        media_type: Option<TelegramMediaType>,
        sender: Option<&str>,
        query: Option<&str>,
    ) -> Vec<TelegramMessage> {
        let q_lower = query.map(|q| q.to_lowercase());
        let s_lower = sender.map(|s| s.to_lowercase());

        chat.messages
            .iter()
            .filter(|m| {
                if let Some(mt) = media_type {
                    if m.media_type != mt {
                        return false;
                    }
                }
                if let Some(ref s) = s_lower {
                    let sender_match = m
                        .sender_name
                        .as_ref()
                        .map(|name| name.to_lowercase().contains(s))
                        .unwrap_or(false);
                    if !sender_match {
                        return false;
                    }
                }
                if let Some(ref q) = q_lower {
                    if !m.text.to_lowercase().contains(q) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }
}
