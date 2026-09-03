use crate::model::SmsMessage;
use anyhow::Result;

pub struct CsvMessageFormatter;

impl CsvMessageFormatter {
    pub fn format(messages: &[SmsMessage]) -> Result<String> {
        let mut out = String::new();
        out.push_str("ID,Address,ContactName,Date,Type,Read,Body\n");

        for msg in messages {
            let type_str = match msg.msg_type {
                crate::model::MessageType::Inbox => "INBOX",
                crate::model::MessageType::Sent => "SENT",
                crate::model::MessageType::Draft => "DRAFT",
                crate::model::MessageType::Outbox => "OUTBOX",
            };

            let escaped_body = msg.body.replace('"', "\"\"").replace('\n', " ");
            let escaped_name = msg.contact_name.as_deref().unwrap_or("").replace('"', "\"\"");

            out.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",{},\"{}\"\n",
                msg.id,
                msg.address,
                escaped_name,
                msg.date.to_rfc3339(),
                type_str,
                msg.read,
                escaped_body
            ));
        }

        Ok(out)
    }
}
