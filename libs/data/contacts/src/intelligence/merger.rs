use super::matcher::{ContactMatcher, MatchConfidence};
use super::normalizer::PhoneNormalizer;
use crate::model::Contact;

pub struct ContactMerger;

impl ContactMerger {
    /// Consolidates duplicate contacts in a list into a deduplicated, merged list.
    pub fn merge_all(contacts: &[Contact], default_country_code: &str) -> Vec<Contact> {
        let mut merged_list: Vec<Contact> = Vec::new();

        for incoming in contacts {
            let mut matched_index = None;
            for (idx, existing) in merged_list.iter().enumerate() {
                let conf = ContactMatcher::match_contacts(existing, incoming, default_country_code);
                if conf == MatchConfidence::Exact || conf == MatchConfidence::High {
                    matched_index = Some(idx);
                    break;
                }
            }

            if let Some(idx) = matched_index {
                let existing = &merged_list[idx];
                let merged = Self::merge_two(existing, incoming, default_country_code);
                merged_list[idx] = merged;
            } else {
                merged_list.push(incoming.clone());
            }
        }

        merged_list
    }

    /// Performs lossless merge of two duplicate contact entries.
    pub fn merge_two(primary: &Contact, secondary: &Contact, default_country_code: &str) -> Contact {
        let mut result = primary.clone();

        // 1. Pick longer/more descriptive display name
        if secondary.display_name.len() > result.display_name.len() {
            result.display_name = secondary.display_name.clone();
        }

        // 2. Structured name fallback
        if result.structured_name.given_name.is_none() && secondary.structured_name.given_name.is_some() {
            result.structured_name = secondary.structured_name.clone();
        }

        // 3. Consolidate Phone Numbers (deduplicate by normalized E.164)
        for p in &secondary.phone_numbers {
            let norm = p.normalized_e164.clone().unwrap_or_else(|| {
                PhoneNormalizer::normalize(&p.raw, default_country_code)
            });
            let already_exists = result.phone_numbers.iter().any(|existing| {
                let en = existing.normalized_e164.clone().unwrap_or_else(|| {
                    PhoneNormalizer::normalize(&existing.raw, default_country_code)
                });
                en == norm
            });
            if !already_exists {
                result.phone_numbers.push(p.clone());
            }
        }

        // 4. Consolidate Emails
        for e in &secondary.emails {
            let clean = e.email.trim().to_lowercase();
            let exists = result.emails.iter().any(|existing| existing.email.trim().to_lowercase() == clean);
            if !exists {
                result.emails.push(e.clone());
            }
        }

        // 5. Consolidate Addresses
        for addr in &secondary.addresses {
            if !result.addresses.contains(addr) {
                result.addresses.push(addr.clone());
            }
        }

        // 6. Organization & Notes fallback
        if result.organization.is_none() && secondary.organization.is_some() {
            result.organization = secondary.organization.clone();
        }
        if result.notes.is_none() && secondary.notes.is_some() {
            result.notes = secondary.notes.clone();
        }
        if result.birthday.is_none() && secondary.birthday.is_some() {
            result.birthday = secondary.birthday.clone();
        }

        // 7. Photos fallback
        if result.photos.is_empty() && !secondary.photos.is_empty() {
            result.photos = secondary.photos.clone();
        }

        result
    }
}
