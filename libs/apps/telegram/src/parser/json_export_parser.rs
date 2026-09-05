use crate::domain::{ChatType, TelegramChat, TelegramMediaType, TelegramMessage};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;

/// Parser for Telegram Desktop JSON export datasets (`result.json`).
pub struct TelegramJsonParser;

impl TelegramJsonParser {
    /// Parses a Telegram Desktop export JSON string into a `TelegramChat`.
    pub fn parse(json_content: &str) -> Option<TelegramChat> {
        let v: Value = serde_json::from_str(json_content).ok()?;

        let chat_id = v.get("id").and_then(|i| i.as_i64()).unwrap_or(1);
        let title = v.get("name").and_then(|n| n.as_str()).unwrap_or("Telegram Chat");
        let type_str = v.get("type").and_then(|t| t.as_str()).unwrap_or("personal_chat");
        let chat_type = ChatType::from_export_str(type_str);

        let mut chat = TelegramChat::new(chat_id, title, chat_type);

        if let Some(Value::Array(msgs)) = v.get("messages") {
            for m in msgs {
                if let Some(msg) = parse_message(m) {
                    chat.add_message(msg);
                }
            }
        }

        Some(chat)
    }
}

fn parse_message(v: &Value) -> Option<TelegramMessage> {
    let id = v.get("id").and_then(|i| i.as_i64())?;
    let date_str = v.get("date").and_then(|d| d.as_str()).unwrap_or("");
    let date = parse_date(date_str);

    let text = match v.get("text") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(arr)) => arr
            .iter()
            .map(|item| match item {
                Value::String(s) => s.as_str(),
                Value::Object(obj) => obj.get("text").and_then(|t| t.as_str()).unwrap_or(""),
                _ => "",
            })
            .collect::<Vec<_>>()
            .join(""),
        _ => String::new(),
    };

    let mut msg = TelegramMessage::new(id, date, text);

    if let Some(from) = v.get("from").and_then(|f| f.as_str()) {
        msg.sender_name = Some(from.to_string());
    }
    if let Some(from_id) = v.get("from_id").and_then(|f| f.as_str()) {
        msg.sender_id = Some(from_id.to_string());
    }

    if let Some(reply_to) = v.get("reply_to_message_id").and_then(|r| r.as_i64()) {
        msg.reply_to_id = Some(reply_to);
    }

    if let Some(media_type_str) = v.get("media_type").and_then(|m| m.as_str()) {
        match media_type_str {
            "voice_message" => msg.media_type = TelegramMediaType::VoiceNote,
            "video_message" => msg.media_type = TelegramMediaType::VideoNote,
            "sticker" => msg.media_type = TelegramMediaType::Sticker,
            "video_file" => msg.media_type = TelegramMediaType::Video,
            "audio_file" => msg.media_type = TelegramMediaType::Audio,
            _ => {}
        }
    } else if let Some(photo) = v.get("photo").and_then(|p| p.as_str()) {
        msg.media_type = TelegramMediaType::Photo;
        msg.media_path = Some(photo.to_string());
    } else if let Some(file) = v.get("file").and_then(|f| f.as_str()) {
        msg.media_path = Some(file.to_string());
        if msg.media_type == TelegramMediaType::TextOnly {
            let ext = std::path::Path::new(file).extension().and_then(|e| e.to_str()).unwrap_or("");
            msg.media_type = TelegramMediaType::from_mime_or_ext(ext);
        }
    }

    if let Some(dur) = v.get("duration_seconds").and_then(|d| d.as_u64()) {
        msg.duration_secs = Some(dur as u32);
    }

    Some(msg)
}

fn parse_date(date_str: &str) -> DateTime<Utc> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return dt.with_timezone(&Utc);
    }
    if let Ok(ndt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S") {
        return DateTime::from_naive_utc_and_offset(ndt, Utc);
    }
    Utc::now()
}
