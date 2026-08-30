/// Common utilities for ADB output parsing
pub struct ParserUtils;

impl ParserUtils {
    pub fn extract_value(line: &str, key: &str) -> Option<String> {
        let key_with_eq = format!("{}=", key);
        if let Some(start) = line.find(&key_with_eq) {
            let value_part = &line[start + key_with_eq.len()..];
            let value = if let Some(end) = value_part.find(", ") {
                value_part[..end].trim().to_string()
            } else {
                value_part.trim().to_string()
            };

            if value.to_lowercase() == "null" || value.is_empty() {
                return None;
            }
            return Some(value);
        }
        None
    }
}
