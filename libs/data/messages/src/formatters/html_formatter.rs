use crate::model::ConversationThread;
use anyhow::Result;

pub struct HtmlTranscriptFormatter;

impl HtmlTranscriptFormatter {
    /// Generates a standalone, beautiful HTML transcript of conversation threads.
    pub fn format_threads(threads: &[ConversationThread]) -> Result<String> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<title>SMS Messages Transcript</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 20px; }\n");
        html.push_str(".thread { background: #1e293b; border-radius: 12px; margin-bottom: 24px; padding: 16px; border: 1px solid #334155; }\n");
        html.push_str(".thread-header { font-size: 1.1rem; font-weight: 600; color: #38bdf8; border-bottom: 1px solid #334155; padding-bottom: 8px; margin-bottom: 12px; }\n");
        html.push_str(".msg { display: flex; flex-direction: column; margin-bottom: 8px; max-width: 75%; }\n");
        html.push_str(".incoming { align-self: flex-start; }\n");
        html.push_str(".incoming .bubble { background: #334155; color: #f1f5f9; border-radius: 12px 12px 12px 2px; padding: 8px 14px; }\n");
        html.push_str(".outgoing { align-self: flex-end; }\n");
        html.push_str(".outgoing .bubble { background: #0284c7; color: #ffffff; border-radius: 12px 12px 2px 12px; padding: 8px 14px; }\n");
        html.push_str(".time { font-size: 0.75rem; color: #94a3b8; margin-top: 4px; }\n");
        html.push_str(".chat-container { display: flex; flex-direction: column; gap: 6px; }\n");
        html.push_str("</style>\n</head>\n<body>\n");
        html.push_str("<h1>💬 SMS Messages Transcript</h1>\n");

        for thread in threads {
            let title = thread.participant.contact_name.as_deref().unwrap_or(&thread.participant.address);
            html.push_str("<div class=\"thread\">\n");
            html.push_str(&format!("<div class=\"thread-header\">{} ({}) - {} messages</div>\n", title, thread.participant.address, thread.messages.len()));
            html.push_str("<div class=\"chat-container\">\n");

            for msg in &thread.messages {
                let cls = if msg.msg_type.is_incoming() { "incoming" } else { "outgoing" };
                let time_str = msg.date.format("%Y-%m-%d %H:%M:%S").to_string();
                html.push_str(&format!(
                    "<div class=\"msg {}\"><div class=\"bubble\">{}</div><div class=\"time\">{}</div></div>\n",
                    cls,
                    Self::escape_html(&msg.body),
                    time_str
                ));
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
