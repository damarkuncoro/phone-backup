use crate::domain::NoteItem;

/// Exporter for generating standalone offline HTML notes wall viewers.
pub struct NotesHtmlExporter;

impl NotesHtmlExporter {
    pub fn export(title: &str, notes: &[NoteItem]) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("<title>{}</title>\n", escape_html(title)));
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; background: #121214; color: #eceff4; margin: 0; padding: 24px; }\n");
        html.push_str(".header { max-width: 1100px; margin: 0 auto 24px auto; display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #2e3440; padding-bottom: 12px; }\n");
        html.push_str(".header h1 { margin: 0; font-size: 22px; color: #88c0d0; }\n");
        html.push_str(".grid { max-width: 1100px; margin: 0 auto; display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 16px; }\n");
        html.push_str(".card { background: #1e1e24; border: 1px solid #2e3440; border-radius: 10px; padding: 16px; display: flex; flex-direction: column; justify-content: space-between; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }\n");
        html.push_str(".card.pinned { border-color: #ebcb8b; }\n");
        html.push_str(".card-title { font-weight: 600; font-size: 16px; margin-bottom: 8px; color: #eceff4; display: flex; justify-content: space-between; }\n");
        html.push_str(".card-body { font-size: 14px; line-height: 1.5; color: #d8dee9; white-space: pre-wrap; margin-bottom: 12px; }\n");
        html.push_str(".checklist { list-style: none; padding: 0; margin: 0 0 12px 0; font-size: 13px; }\n");
        html.push_str(".checklist li { padding: 3px 0; display: flex; align-items: center; gap: 8px; }\n");
        html.push_str(".checklist li.checked { text-decoration: line-through; color: #4c566a; }\n");
        html.push_str(".tags { display: flex; flex-wrap: wrap; gap: 6px; margin-top: auto; }\n");
        html.push_str(".tag { font-size: 11px; background: #2e3440; color: #81a1c1; padding: 2px 8px; border-radius: 12px; }\n");
        html.push_str(".card-footer { font-size: 11px; color: #4c566a; margin-top: 10px; text-align: right; }\n");
        html.push_str("</style>\n</head>\n<body>\n");

        html.push_str("<div class=\"header\">\n");
        html.push_str(&format!("<h1>{}</h1>\n", escape_html(title)));
        html.push_str(&format!("<span>{} notes archived</span>\n", notes.len()));
        html.push_str("</div>\n");

        html.push_str("<div class=\"grid\">\n");
        for note in notes {
            let pin_class = if note.is_pinned { "card pinned" } else { "card" };
            html.push_str(&format!("<div class=\"{}\">\n<div>\n", pin_class));
            html.push_str("<div class=\"card-title\">\n");
            html.push_str(&format!("<span>{}</span>\n", escape_html(&note.title)));
            if note.is_pinned {
                html.push_str("<span>📌</span>\n");
            }
            html.push_str("</div>\n");

            if !note.content.is_empty() {
                html.push_str(&format!("<div class=\"card-body\">{}</div>\n", escape_html(&note.content)));
            }

            if !note.checklist_items.is_empty() {
                html.push_str("<ul class=\"checklist\">\n");
                for item in &note.checklist_items {
                    let (check_str, class_str) = if item.is_checked { ("☑", "class=\"checked\"") } else { ("☐", "") };
                    html.push_str(&format!("<li {}><span>{}</span> {}</li>\n", class_str, check_str, escape_html(&item.text)));
                }
                html.push_str("</ul>\n");
            }
            html.push_str("</div>\n");

            if !note.tags.is_empty() {
                html.push_str("<div class=\"tags\">\n");
                for tag in &note.tags {
                    html.push_str(&format!("<span class=\"tag\">#{}</span>\n", escape_html(tag)));
                }
                html.push_str("</div>\n");
            }

            html.push_str(&format!("<div class=\"card-footer\">Updated {}</div>\n", note.updated_at.format("%Y-%m-%d %H:%M")));
            html.push_str("</div>\n");
        }
        html.push_str("</div>\n</body>\n</html>");

        html
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
