use crate::model::{ChatType, WhatsAppChat, WhatsAppMessage};
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};

pub struct WhatsAppTxtParser;

impl WhatsAppTxtParser {
    /// Parse raw exported WhatsApp .txt transcript into a structured `WhatsAppChat`
    pub fn parse(chat_name: &str, raw_text: &str) -> Result<WhatsAppChat> {
        let jid = format!("{}@s.whatsapp.net", chat_name.to_lowercase().replace(' ', "_"));
        let mut chat = WhatsAppChat::new(jid, Some(chat_name.to_string()), ChatType::Individual);

        let mut current_sender: Option<String> = None;
        let mut current_time: Option<DateTime<Utc>> = None;
        let mut current_body_lines: Vec<String> = Vec::new();
        let mut msg_counter = 0;

        let flush_message = |chat: &mut WhatsAppChat,
                             sender: &Option<String>,
                             time: &Option<DateTime<Utc>>,
                             lines: &mut Vec<String>,
                             counter: &mut usize| {
            if let (Some(sender_name), Some(timestamp)) = (sender.as_ref(), time.as_ref()) {
                let body = lines.join("\n").trim().to_string();
                if !body.is_empty() {
                    *counter += 1;
                    let from_me = sender_name.eq_ignore_ascii_case("anda")
                        || sender_name.eq_ignore_ascii_case("you");
                    let msg = WhatsAppMessage::new_text(
                        format!("msg_{}", counter),
                        chat.jid.clone(),
                        sender_name.clone(),
                        from_me,
                        *timestamp,
                        body,
                    );
                    chat.add_message(msg);
                }
            }
            lines.clear();
        };

        for line in raw_text.lines() {
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                continue;
            }

            if let Some((date_str, time_str, sender, message)) = Self::parse_line_header(trimmed) {
                flush_message(&mut chat, &current_sender, &current_time, &mut current_body_lines, &mut msg_counter);

                current_sender = Some(sender.to_string());
                current_time = Some(Self::parse_timestamp(&date_str, &time_str));
                current_body_lines.push(message.to_string());
            } else {
                // Continuation line
                if current_sender.is_some() {
                    current_body_lines.push(trimmed.to_string());
                }
            }
        }

        flush_message(&mut chat, &current_sender, &current_time, &mut current_body_lines, &mut msg_counter);

        Ok(chat)
    }

    fn parse_line_header(line: &str) -> Option<(String, String, String, String)> {
        // iOS Format: [14/07/26, 10.30.00] Siti Rahma: Selamat pagi Pak
        if line.starts_with('[') {
            if let Some(close_idx) = line.find(']') {
                let header = &line[1..close_idx];
                let rest = line[close_idx + 1..].trim_start();
                if let Some(colon_idx) = rest.find(':') {
                    let sender = rest[..colon_idx].trim();
                    let msg = rest[colon_idx + 1..].trim_start();
                    let parts: Vec<&str> = header.split(&[',', ' '][..]).filter(|s| !s.is_empty()).collect();
                    if parts.len() >= 2 {
                        return Some((parts[0].to_string(), parts[1].to_string(), sender.to_string(), msg.to_string()));
                    }
                }
            }
        }

        // Android Format: 12/08/2026, 09:16 - Budi Pratama: Halo mas, file backup sudah siap?
        if let Some(dash_idx) = line.find(" - ") {
            let prefix = &line[..dash_idx];
            let after_dash = &line[dash_idx + 3..];

            if let Some(colon_idx) = after_dash.find(':') {
                let sender = after_dash[..colon_idx].trim();
                let msg = after_dash[colon_idx + 1..].trim_start();
                let prefix_parts: Vec<&str> = prefix.split(&[',', ' '][..]).filter(|s| !s.is_empty()).collect();
                if prefix_parts.len() >= 2 && prefix_parts[0].contains('/') {
                    return Some((prefix_parts[0].to_string(), prefix_parts[1].to_string(), sender.to_string(), msg.to_string()));
                }
            }
        }

        None
    }

    fn parse_timestamp(date_str: &str, time_str: &str) -> DateTime<Utc> {
        let clean_time = time_str.replace('.', ":");
        let date_parts: Vec<&str> = date_str.split('/').collect();
        if date_parts.len() == 3 {
            let (d, m, y) = (date_parts[0], date_parts[1], date_parts[2]);
            let full_year = if y.len() == 2 {
                format!("20{}", y)
            } else {
                y.to_string()
            };
            let time_parts: Vec<&str> = clean_time.split(':').collect();
            let hour = time_parts.first().unwrap_or(&"00");
            let minute = time_parts.get(1).unwrap_or(&"00");
            let second = time_parts.get(2).unwrap_or(&"00");

            let formatted = format!("{}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}", full_year, m, d, hour, minute, second);
            if let Ok(naive) = NaiveDateTime::parse_from_str(&formatted, "%Y-%m-%d %H:%M:%S") {
                return naive.and_utc();
            }
        }
        Utc::now()
    }
}
