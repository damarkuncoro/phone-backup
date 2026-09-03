use crate::model::WhatsAppChat;
use anyhow::Result;

pub struct WhatsAppHtmlViewer;

impl WhatsAppHtmlViewer {
    /// Renders chats into an interactive, WhatsApp-themed HTML archive.
    pub fn render_archive(chats: &[WhatsAppChat]) -> Result<String> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<title>WhatsApp Archive Viewer</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0b141a; color: #e9edef; margin: 0; padding: 20px; }\n");
        html.push_str(".header { background: #202c33; padding: 16px; border-radius: 12px; margin-bottom: 24px; border-left: 5px solid #00a884; }\n");
        html.push_str(".header h1 { margin: 0; font-size: 1.5rem; color: #00a884; }\n");
        html.push_str(".chat-box { background: #111b21; border-radius: 12px; margin-bottom: 24px; padding: 16px; border: 1px solid #222e35; }\n");
        html.push_str(".chat-title { font-weight: bold; color: #53bdeb; margin-bottom: 12px; padding-bottom: 8px; border-bottom: 1px solid #222e35; }\n");
        html.push_str(".messages-container { display: flex; flex-direction: column; gap: 8px; }\n");
        html.push_str(".msg { display: flex; flex-direction: column; max-width: 70%; padding: 8px 12px; border-radius: 8px; font-size: 0.95rem; }\n");
        html.push_str(".from-me { align-self: flex-end; background: #005c4b; color: #e9edef; border-top-right-radius: 0; }\n");
        html.push_str(".from-other { align-self: flex-start; background: #202c33; color: #e9edef; border-top-left-radius: 0; }\n");
        html.push_str(".sender-name { font-size: 0.75rem; color: #53bdeb; font-weight: bold; margin-bottom: 2px; }\n");
        html.push_str(".time { font-size: 0.7rem; color: #8696a0; align-self: flex-end; margin-top: 4px; }\n");
        html.push_str("</style>\n</head>\n<body>\n");
        html.push_str("<div class=\"header\"><h1>📱 WhatsApp Offline Archive</h1></div>\n");

        for chat in chats {
            let chat_name = chat.name.as_deref().unwrap_or(&chat.jid);
            html.push_str("<div class=\"chat-box\">\n");
            html.push_str(&format!("<div class=\"chat-title\">{} ({} messages)</div>\n", Self::escape_html(chat_name), chat.messages.len()));
            html.push_str("<div class=\"messages-container\">\n");

            for msg in &chat.messages {
                let cls = if msg.from_me { "from-me" } else { "from-other" };
                let time_str = msg.timestamp.format("%Y-%m-%d %H:%M").to_string();

                html.push_str(&format!("<div class=\"msg {}\">\n", cls));
                if !msg.from_me {
                    if let Some(sname) = &msg.sender_name {
                        html.push_str(&format!("<div class=\"sender-name\">{}</div>\n", Self::escape_html(sname)));
                    }
                }
                html.push_str(&format!("<div>{}</div>\n", Self::escape_html(&msg.body)));
                html.push_str(&format!("<div class=\"time\">{}</div>\n", time_str));
                html.push_str("</div>\n");
            }

            html.push_str("</div>\n</div>\n");
        }

        html.push_str("</body>\n</html>\n");
        Ok(html)
    }

    fn escape_html(s: &str) -> String {
        s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
    }
}
