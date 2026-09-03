use super::csv_formatter::CsvFormatter;
use super::json_formatter::JsonFormatter;
use crate::model::Contact;
use crate::vcard::{VCardVersion, VCardWriter};
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    VCard3,
    VCard4,
    VCard21,
    Csv,
    Json,
    Ndjson,
}

pub struct ContactFormatterFactory;

impl ContactFormatterFactory {
    /// Formats contacts into the requested export format string.
    pub fn format_contacts(contacts: &[Contact], format: ExportFormat) -> Result<String> {
        match format {
            ExportFormat::VCard3 => Ok(VCardWriter::write_contacts(contacts, VCardVersion::V3_0)),
            ExportFormat::VCard4 => Ok(VCardWriter::write_contacts(contacts, VCardVersion::V4_0)),
            ExportFormat::VCard21 => Ok(VCardWriter::write_contacts(contacts, VCardVersion::V2_1)),
            ExportFormat::Csv => Ok(CsvFormatter::to_csv(contacts)),
            ExportFormat::Json => JsonFormatter::to_json(contacts),
            ExportFormat::Ndjson => JsonFormatter::to_ndjson(contacts),
        }
    }
}
