use crate::domain::{TelegramChat, TelegramMediaType};

/// Exporter for generating standalone offline HTML chat archive viewers.
pub struct TelegramHtmlExporter;

impl TelegramHtmlExporter {
    pub fn export(chat: &TelegramChat) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("<title>Telegram Backup: {}</title>\n", escape_html(&chat.title)));
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; background: #0f141c; color: #f5f5f5; margin: 0; padding: 20px; }\n");
        html.push_str(".chat-container { max-width: 760px; margin: 0 auto; background: #17212b; border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.5); overflow: hidden; }\n");
        html.push_str(".header { background: #242f3d; padding: 16px 20px; border-bottom: 1px solid #2b394a; }\n");
        html.push_str(".header h1 { margin: 0; font-size: 18px; color: #5288c1; }\n");
        html.push_str(".header span { font-size: 13px; color: #7f91a4; }\n");
        html.push_str(".messages { padding: 20px; display: flex; flex-direction: column; gap: 12px; }\n");
        html.push_str(".msg { max-width: 75%; background: #182533; border-radius: 8px; padding: 10px 14px; position: relative; word-break: break-word; }\n");
        html.push_str(".sender { font-size: 13px; font-weight: 600; color: #64b5f6; margin-bottom: 4px; }\n");
        html.push_str(".text { font-size: 14px; line-height: 1.4; }\n");
        html.push_str(".media-badge { display: inline-block; background: #2b5278; color: #e3f2fd; font-size: 11px; padding: 2px 8px; border-radius: 4px; margin-top: 6px; }\n");
        html.push_str(".date { font-size: 11px; color: #6c7883; text-align: right; margin-top: 4px; }\n");
        html.push_str("</style>\n</head>\n<body>\n");

        html.push_str("<div class=\"chat-container\">\n");
        html.push_str("<div class=\"header\">\n");
        html.push_str(&format!("<h1>{}</h1>\n", escape_html(&chat.title)));
        html.push_str(&format!("<span>{} &bull; {} messages</span>\n", chat.chat_type, chat.total_messages()));
        html.push_str("</div>\n");

        html.push_str("<div class=\"messages\">\n");
        for m in &chat.messages {
            html.push_str("<div class=\"msg\">\n");
            if let Some(ref sender) = m.sender_name {
                html.push_str(&format!("<div class=\"sender\">{}</div>\n", escape_html(sender)));
            }
            if !m.text.is_empty() {
                html.push_str(&format!("<div class=\"text\">{}</div>\n", escape_html(&m.text)));
            }
            if m.has_media() {
                let badge = match m.media_type {
                    TelegramMediaType::VoiceNote => format!("🎙️ Voice Note ({}s)", m.duration_secs.unwrap_or(0)),
                    TelegramMediaType::VideoNote => format!("📹 Video Note (Round, {}s)", m.duration_secs.unwrap_or(0)),
                    TelegramMediaType::Photo => "📷 Photo".to_string(),
                    TelegramMediaType::Video => "🎥 Video".to_string(),
                    TelegramMediaType::Sticker => "🎨 Sticker".to_string(),
                    TelegramMediaType::Document => "📄 Document".to_string(),
                    _ => "📎 Attachment".to_string(),
                };
                html.push_str(&format!("<div class=\"media-badge\">{}</div>\n", badge));
            }
            html.push_str(&format!("<div class=\"date\">{}</div>\n", m.date.format("%Y-%m-%d %H:%M")));
            html.push_str("</div>\n");
        }
        html.push_str("</div>\n</div>\n</body>\n</html>");

        html
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
