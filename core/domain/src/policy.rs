use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPolicy {
    pub include_paths: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub min_file_size: Option<u64>,
    pub max_file_size: Option<u64>,
}

impl Default for BackupPolicy {
    fn default() -> Self {
        Self {
            include_paths: vec!["/sdcard/".to_string()],
            exclude_patterns: vec![
                "*/.cache/*".to_string(),
                "*/cache/*".to_string(),
                "*/tmp/*".to_string(),
                "*.tmp".to_string(),
            ],
            min_file_size: None,
            max_file_size: None,
        }
    }
}

impl BackupPolicy {
    pub fn should_include(&self, path: &str) -> bool {
        // 1. Check Include Paths (if any)
        if !self.include_paths.is_empty() {
            let mut included = false;
            for inc in &self.include_paths {
                if path.starts_with(inc) {
                    included = true;
                    break;
                }
            }
            if !included {
                return false;
            }
        }

        // 2. Check Exclude Patterns
        for pattern in &self.exclude_patterns {
            let p = pattern.replace("*", "");
            if !p.is_empty() && path.contains(&p) {
                return false;
            }
        }
        true
    }
}
