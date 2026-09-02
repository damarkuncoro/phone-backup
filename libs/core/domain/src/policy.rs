use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPolicy {
    pub include_paths: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

impl Default for BackupPolicy {
    fn default() -> Self {
        BackupPolicyBuilder::new().build()
    }
}

impl BackupPolicy {
    pub fn builder() -> BackupPolicyBuilder {
        BackupPolicyBuilder::new()
    }

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
            if let Some(ext) = pattern.strip_prefix("*.") {
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

#[derive(Debug, Clone)]
pub struct BackupPolicyBuilder {
    include_paths: Vec<String>,
    exclude_patterns: Vec<String>,
}

impl BackupPolicyBuilder {
    pub fn new() -> Self {
        Self {
            include_paths: Vec::new(),
            exclude_patterns: vec![
                "*.tmp".to_string(),
                "*.cache".to_string(),
                "cache/".to_string(),
                ".thumbnails/".to_string(),
            ],
        }
    }

    pub fn include(mut self, path: &str) -> Self {
        self.include_paths.push(path.to_string());
        self
    }

    pub fn include_many(mut self, paths: Vec<String>) -> Self {
        self.include_paths.extend(paths);
        self
    }

    pub fn exclude(mut self, pattern: &str) -> Self {
        self.exclude_patterns.push(pattern.to_string());
        self
    }

    pub fn exclude_many(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns.extend(patterns);
        self
    }

    pub fn build(self) -> BackupPolicy {
        BackupPolicy {
            include_paths: self.include_paths,
            exclude_patterns: self.exclude_patterns,
        }
    }
}

impl Default for BackupPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
