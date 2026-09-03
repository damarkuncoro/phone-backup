use crate::formatters::{MessageExportFormat, MessageFormatterFactory};
use crate::intelligence::{CallLogAnalytics, CallStatsSummary, MessageCategory, MessageClassifier};
use crate::model::{CallEntry, CallType, ConversationThread, MessageType, SmsMessage};
use crate::threading::ThreadEngine;
use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct SmsMessageBuilder {
    msg: SmsMessage,
}

impl SmsMessageBuilder {
    pub fn new(id: impl Into<String>, address: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            msg: SmsMessage::new(id, address, body, Utc::now(), MessageType::Inbox),
        }
    }

    pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
        self.msg.date = date;
        self
    }

    pub fn with_type(mut self, msg_type: MessageType) -> Self {
        self.msg.msg_type = msg_type;
        self
    }

    pub fn with_contact_name(mut self, name: impl Into<String>) -> Self {
        self.msg.contact_name = Some(name.into());
        self
    }

    pub fn with_read(mut self, read: bool) -> Self {
        self.msg.read = read;
        self
    }

    pub fn build(self) -> SmsMessage {
        self.msg
    }
}

pub struct CallEntryBuilder {
    call: CallEntry,
}

impl CallEntryBuilder {
    pub fn new(id: impl Into<String>, number: impl Into<String>) -> Self {
        Self {
            call: CallEntry::new(id, number, Utc::now(), 0, CallType::Incoming),
        }
    }

    pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
        self.call.date = date;
        self
    }

    pub fn with_duration(mut self, duration_seconds: u64) -> Self {
        self.call.duration_seconds = duration_seconds;
        self
    }

    pub fn with_type(mut self, call_type: CallType) -> Self {
        self.call.call_type = call_type;
        self
    }

    pub fn with_contact_name(mut self, name: impl Into<String>) -> Self {
        self.call.contact_name = Some(name.into());
        self
    }

    pub fn build(self) -> CallEntry {
        self.call
    }
}

pub struct MessageStore {
    pub messages: Vec<SmsMessage>,
    pub calls: Vec<CallEntry>,
}

impl MessageStore {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            calls: Vec::new(),
        }
    }

    pub fn add_message(&mut self, msg: SmsMessage) {
        self.messages.push(msg);
    }

    pub fn add_call(&mut self, call: CallEntry) {
        self.calls.push(call);
    }

    pub fn build_threads(&self) -> Vec<ConversationThread> {
        ThreadEngine::build_threads(self.messages.clone())
    }

    pub fn call_stats(&self) -> CallStatsSummary {
        CallLogAnalytics::compute_summary(&self.calls)
    }

    pub fn filter_by_category(&self, category: MessageCategory) -> Vec<SmsMessage> {
        self.messages
            .iter()
            .filter(|m| MessageClassifier::classify(&m.address, &m.body) == category)
            .cloned()
            .collect()
    }

    pub fn export_messages(&self, format: MessageExportFormat) -> Result<String> {
        MessageFormatterFactory::export(&self.messages, format)
    }
}

impl Default for MessageStore {
    fn default() -> Self {
        Self::new()
    }
}
