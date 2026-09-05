/// International Phone Number Normalizer with E.164 standard formatting.
pub struct PhoneNormalizer;

impl PhoneNormalizer {
    /// Normalizes raw phone number string to canonical E.164 format.
    /// `default_country_code` is the dialing prefix (e.g. "+62" for Indonesia, "+1" for US).
    pub fn normalize(raw: &str, default_country_code: &str) -> String {
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect();

        if cleaned.is_empty() {
            return String::new();
        }

        let prefix = if default_country_code.starts_with('+') {
            default_country_code.to_string()
        } else {
            format!("+{}", default_country_code)
        };

        let digits_only_prefix = prefix.trim_start_matches('+');

        if cleaned.starts_with('+') {
            cleaned
        } else if let Some(stripped) = cleaned.strip_prefix("00") {
            format!("+{}", stripped)
        } else if cleaned.starts_with(digits_only_prefix) {
            format!("+{}", cleaned)
        } else if let Some(stripped) = cleaned.strip_prefix('0') {
            format!("{}{}", prefix, stripped)
        } else {
            format!("{}{}", prefix, cleaned)
        }
    }

    /// Strips non-digit formatting characters for comparison.
    pub fn strip_delimiters(raw: &str) -> String {
        raw.chars().filter(|c| c.is_ascii_digit()).collect()
    }
}
