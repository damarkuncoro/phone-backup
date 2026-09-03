use crate::model::{ConversationThread, SmsMessage};
use std::collections::HashMap;

pub struct ThreadEngine;

impl ThreadEngine {
    /// Groups messages by sender/receiver address into structured conversational threads.
    pub fn build_threads(messages: Vec<SmsMessage>) -> Vec<ConversationThread> {
        let mut threads_map: HashMap<String, ConversationThread> = HashMap::new();

        for msg in messages {
            let normalized_address = Self::normalize_address(&msg.address);
            let thread = threads_map
                .entry(normalized_address.clone())
                .or_insert_with(|| {
                    ConversationThread::new(normalized_address, &msg.address, msg.contact_name.clone())
                });

            thread.add_message(msg);
        }

        let mut thread_list: Vec<ConversationThread> = threads_map.into_values().collect();

        // Sort messages in each thread chronologically (oldest to newest)
        for thread in &mut thread_list {
            thread.messages.sort_by_key(|m| m.date);
        }

        // Sort threads by latest message date (newest thread first)
        thread_list.sort_by(|a, b| b.last_message_date.cmp(&a.last_message_date));

        thread_list
    }

    fn normalize_address(address: &str) -> String {
        let digits: String = address.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() >= 9 {
            digits[digits.len() - 9..].to_string()
        } else {
            address.trim().to_lowercase()
        }
    }
}
