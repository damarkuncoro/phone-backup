use super::normalizer::PhoneNormalizer;
use crate::model::Contact;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchConfidence {
    Exact,
    High,
    Medium,
    None,
}

pub struct ContactMatcher;

impl ContactMatcher {
    /// Compares two contacts and returns the confidence level of them being duplicates.
    pub fn match_contacts(a: &Contact, b: &Contact, default_country_code: &str) -> MatchConfidence {
        // 1. Check matching normalized phone numbers
        for pa in &a.phone_numbers {
            let na = pa.normalized_e164.clone().unwrap_or_else(|| {
                PhoneNormalizer::normalize(&pa.raw, default_country_code)
            });
            if na.is_empty() {
                continue;
            }
            for pb in &b.phone_numbers {
                let nb = pb.normalized_e164.clone().unwrap_or_else(|| {
                    PhoneNormalizer::normalize(&pb.raw, default_country_code)
                });
                if !nb.is_empty() && na == nb {
                    return MatchConfidence::Exact;
                }
            }
        }

        // 2. Check matching email addresses
        for ea in &a.emails {
            let ea_clean = ea.email.trim().to_lowercase();
            if ea_clean.is_empty() {
                continue;
            }
            for eb in &b.emails {
                let eb_clean = eb.email.trim().to_lowercase();
                if !eb_clean.is_empty() && ea_clean == eb_clean {
                    return MatchConfidence::Exact;
                }
            }
        }

        // 3. Exact Display Name match
        let name_a = a.display_name.trim().to_lowercase();
        let name_b = b.display_name.trim().to_lowercase();
        if !name_a.is_empty() && name_a == name_b {
            return MatchConfidence::High;
        }

        // 4. Fuzzy Levenshtein Distance match on names
        if !name_a.is_empty() && !name_b.is_empty() {
            let sim = Self::name_similarity(&name_a, &name_b);
            if sim >= 0.85 {
                return MatchConfidence::Medium;
            }
        }

        MatchConfidence::None
    }

    /// Fast normalized Levenshtein similarity metric [0.0 - 1.0]
    pub fn name_similarity(s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }
        let len1 = s1.chars().count();
        let len2 = s2.chars().count();
        let max_len = len1.max(len2);
        if max_len == 0 {
            return 1.0;
        }
        let dist = Self::levenshtein_distance(s1, s2);
        1.0 - (dist as f64 / max_len as f64)
    }

    fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let v1: Vec<char> = s1.chars().collect();
        let v2: Vec<char> = s2.chars().collect();
        let mut prev = (0..=v2.len()).collect::<Vec<usize>>();
        let mut curr = vec![0; v2.len() + 1];

        for (i, c1) in v1.iter().enumerate() {
            curr[0] = i + 1;
            for (j, c2) in v2.iter().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                curr[j + 1] = (curr[j] + 1)
                    .min(prev[j + 1] + 1)
                    .min(prev[j] + cost);
            }
            prev.copy_from_slice(&curr);
        }
        prev[v2.len()]
    }
}
