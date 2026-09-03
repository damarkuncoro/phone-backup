use crate::builder::ContactBookBuilder;
use crate::formatters::{ContactFormatterFactory, ExportFormat};
use crate::intelligence::{ContactDiff, ContactDiffEngine, ContactMerger, PhoneNormalizer};
use crate::model::Contact;
use crate::vcard::VCardParser;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ContactBook {
    pub contacts: Vec<Contact>,
    pub default_country_code: String,
}

impl ContactBook {
    pub fn new(contacts: Vec<Contact>, default_country_code: impl Into<String>) -> Self {
        Self {
            contacts,
            default_country_code: default_country_code.into(),
        }
    }

    pub fn builder() -> ContactBookBuilder {
        ContactBookBuilder::new()
    }

    pub fn len(&self) -> usize {
        self.contacts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contacts.is_empty()
    }

    pub fn from_vcard(vcard_str: &str, default_country_code: &str) -> Result<Self> {
        let contacts = VCardParser::parse_str(vcard_str)?;
        Ok(Self::new(contacts, default_country_code))
    }

    /// Normalizes all phone numbers in the book to E.164.
    pub fn normalize_phone_numbers(&mut self) {
        for c in &mut self.contacts {
            for p in &mut c.phone_numbers {
                p.normalized_e164 = Some(PhoneNormalizer::normalize(&p.raw, &self.default_country_code));
            }
        }
    }

    /// Deduplicates and losslessly merges all duplicate entries in the book.
    pub fn deduplicate(&mut self) {
        self.contacts = ContactMerger::merge_all(&self.contacts, &self.default_country_code);
    }

    /// Computes diff compared to another ContactBook.
    pub fn diff(&self, other: &ContactBook) -> ContactDiff {
        ContactDiffEngine::compute_diff(&self.contacts, &other.contacts, &self.default_country_code)
    }

    /// Exports entire book to requested format (vCard 3/4, CSV, JSON, NDJSON).
    pub fn export(&self, format: ExportFormat) -> Result<String> {
        ContactFormatterFactory::format_contacts(&self.contacts, format)
    }
}
