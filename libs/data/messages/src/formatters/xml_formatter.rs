use crate::model::{MessageType, SmsMessage};
use anyhow::Result;

pub struct XmlSmsBackupFormatter;

impl XmlSmsBackupFormatter {
    /// Formats SMS list into standard Android "SMS Backup & Restore" XML format.
    pub fn format(messages: &[SmsMessage]) -> Result<String> {
        let mut out = String::new();
        out.push_str("<?xml version='1.0' encoding='UTF-8' standalone='yes' ?>\n");
        out.push_str(&format!("<smses count=\"{}\">\n", messages.len()));

        for msg in messages {
            let msg_type_int = match msg.msg_type {
                MessageType::Inbox => 1,
                MessageType::Sent => 2,
                MessageType::Draft => 3,
                MessageType::Outbox => 4,
            };

            let timestamp_millis = msg.date.timestamp_millis();
            let escaped_body = Self::escape_xml(&msg.body);
            let escaped_addr = Self::escape_xml(&msg.address);
            let escaped_name = Self::escape_xml(msg.contact_name.as_deref().unwrap_or("(Unknown)"));

            out.push_str(&format!(
                "  <sms protocol=\"0\" address=\"{}\" date=\"{}\" type=\"{}\" subject=\"null\" body=\"{}\" toa=\"null\" sc_toa=\"null\" service_center=\"{}\" read=\"{}\" status=\"-1\" locked=\"0\" contact_name=\"{}\" />\n",
                escaped_addr,
                timestamp_millis,
                msg_type_int,
                escaped_body,
                msg.service_center.as_deref().unwrap_or("null"),
                if msg.read { 1 } else { 0 },
                escaped_name
            ));
        }

        out.push_str("</smses>\n");
        Ok(out)
    }

    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}
