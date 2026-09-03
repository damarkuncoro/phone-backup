use super::matcher::{ContactMatcher, MatchConfidence};
use crate::model::Contact;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactDiff {
    pub added: Vec<Contact>,
    pub modified: Vec<(Contact, Contact)>, // (old, new)
    pub deleted: Vec<Contact>,
    pub unchanged_count: usize,
}

pub struct ContactDiffEngine;

impl ContactDiffEngine {
    /// Compares old snapshot contacts vs new snapshot contacts to compute added, modified, deleted.
    pub fn compute_diff(old_contacts: &[Contact], new_contacts: &[Contact], country_code: &str) -> ContactDiff {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut matched_old_indices = vec![false; old_contacts.len()];
        let mut unchanged_count = 0;

        for new_c in new_contacts {
            let mut match_found = false;

            for (old_idx, old_c) in old_contacts.iter().enumerate() {
                if matched_old_indices[old_idx] {
                    continue;
                }

                let conf = ContactMatcher::match_contacts(old_c, new_c, country_code);
                if conf == MatchConfidence::Exact || conf == MatchConfidence::High {
                    matched_old_indices[old_idx] = true;
                    match_found = true;

                    if old_c == new_c {
                        unchanged_count += 1;
                    } else {
                        modified.push((old_c.clone(), new_c.clone()));
                    }
                    break;
                }
            }

            if !match_found {
                added.push(new_c.clone());
            }
        }

        let mut deleted = Vec::new();
        for (idx, matched) in matched_old_indices.iter().enumerate() {
            if !*matched {
                deleted.push(old_contacts[idx].clone());
            }
        }

        ContactDiff {
            added,
            modified,
            deleted,
            unchanged_count,
        }
    }
}
