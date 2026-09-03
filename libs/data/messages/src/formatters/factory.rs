use super::csv_formatter::CsvMessageFormatter;
use super::html_formatter::HtmlTranscriptFormatter;
use super::json_formatter::JsonMessageFormatter;
use super::xml_formatter::XmlSmsBackupFormatter;
use crate::model::SmsMessage;
use crate::threading::ThreadEngine;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageExportFormat {
    Xml,
    Html,
    Csv,
    Json,
    Ndjson,
}

pub struct MessageFormatterFactory;

impl MessageFormatterFactory {
    pub fn export(messages: &[SmsMessage], format: MessageExportFormat) -> Result<String> {
        match format {
            MessageExportFormat::Xml => XmlSmsBackupFormatter::format(messages),
            MessageExportFormat::Html => {
                let threads = ThreadEngine::build_threads(messages.to_vec());
                HtmlTranscriptFormatter::format_threads(&threads)
            }
            MessageExportFormat::Csv => CsvMessageFormatter::format(messages),
            MessageExportFormat::Json => JsonMessageFormatter::format_pretty(messages),
            MessageExportFormat::Ndjson => JsonMessageFormatter::format_ndjson(messages),
        }
    }
}
