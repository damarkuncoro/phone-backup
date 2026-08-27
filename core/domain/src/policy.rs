use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPolicy {
    pub include_paths: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

impl Default for BackupPolicy {
    fn default() -> Self {
        Self {
            include_paths: vec![],
            exclude_patterns: vec![
                "*.tmp".to_string(),
                "*.cache".to_string(),
                "cache/".to_string(),
                ".thumbnails/".to_string(),
            ],
        }
    }
}

impl BackupPolicy {
    pub fn should_include(&self, path: &str) -> bool {
        // If include_paths is not empty, path must start with one of them
        if !self.include_paths.is_empty() {
            let matches_include = self.include_paths.iter().any(|p| path.starts_with(p));
            if !matches_include {
                return false;
            }
        }

        // Check against exclude patterns
        for pattern in &self.exclude_patterns {
            if pattern.starts_with("*.") {
                let ext = &pattern[2..];
                if path.ends_with(ext) {
                    return false;
                }
            } else if path.contains(pattern) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_policy() {
        let mut policy = BackupPolicy::default();
        policy.include_paths = vec!["/sdcard/DCIM".to_string()];

        assert!(policy.should_include("/sdcard/DCIM/photo.jpg"));
        assert!(!policy.should_include("/sdcard/Downloads/file.pdf"));
    }

    #[test]
    fn test_exclude_policy() {
        let mut policy = BackupPolicy::default();
        policy.exclude_patterns = vec!["*.tmp".to_string(), "cache/".to_string()];

        assert!(!policy.should_include("/sdcard/data.tmp"));
        assert!(!policy.should_include("/sdcard/Android/cache/info.log"));
        assert!(policy.should_include("/sdcard/Documents/notes.txt"));
    }
}
