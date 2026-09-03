use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtpResult {
    pub code: String,
    pub service_name: Option<String>,
}

pub struct OtpExtractor;

impl OtpExtractor {
    /// Extracts 4 to 8 digit numeric verification code from SMS message body.
    pub fn extract(address: &str, body: &str) -> Option<OtpResult> {
        let words: Vec<&str> = body.split_whitespace().collect();

        // Find candidate code (standalone 4-8 digits)
        let mut found_code = None;
        for word in words {
            let clean = word.trim_matches(|c: char| !c.is_ascii_digit());
            if (4..=8).contains(&clean.len()) && clean.chars().all(|c| c.is_ascii_digit()) {
                found_code = Some(clean.to_string());
                break;
            }
        }

        let code = found_code?;
        let service_name = Self::guess_service(address, body);

        Some(OtpResult {
            code,
            service_name,
        })
    }

    fn guess_service(address: &str, body: &str) -> Option<String> {
        let combined = format!("{} {}", address, body).to_lowercase();
        let services = [
            ("google", "Google"),
            ("whatsapp", "WhatsApp"),
            ("telegram", "Telegram"),
            ("bca", "BCA"),
            ("mandiri", "Mandiri"),
            ("tokopedia", "Tokopedia"),
            ("shopee", "Shopee"),
            ("gojek", "Gojek"),
            ("grab", "Grab"),
            ("dana", "DANA"),
            ("ovo", "OVO"),
        ];

        for (pattern, name) in services {
            if combined.contains(pattern) {
                return Some(name.to_string());
            }
        }

        if !address.chars().all(|c| c.is_ascii_digit() || c == '+') {
            return Some(address.to_string());
        }

        None
    }
}
