use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of Telegram chat dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChatType {
    PersonalChat,
    Group,
    Supergroup,
    Channel,
    BotChat,
    Unknown,
}

impl ChatType {
    pub fn from_export_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "personal_chat" | "personal" | "private" => Self::PersonalChat,
            "private_group" | "group" => Self::Group,
            "private_supergroup" | "public_supergroup" | "supergroup" => Self::Supergroup,
            "private_channel" | "public_channel" | "channel" => Self::Channel,
            "bot_chat" | "bot" => Self::BotChat,
            _ => Self::Unknown,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::PersonalChat => "Direct Chat",
            Self::Group => "Group",
            Self::Supergroup => "Supergroup",
            Self::Channel => "Channel",
            Self::BotChat => "Bot",
            Self::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for ChatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
