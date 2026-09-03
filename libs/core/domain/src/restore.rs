use crate::crypto::EncryptionMode;
use serde::{Deserialize, Serialize};

/// Options for restoring a backup snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreOptions {
    pub target_dir: String,
    pub encryption: EncryptionMode,
    pub filters: Option<Vec<String>>,
    pub overwrite_existing: bool,
}

impl RestoreOptions {
    pub fn new(target_dir: impl Into<String>) -> Self {
        Self {
            target_dir: target_dir.into(),
            encryption: EncryptionMode::None,
            filters: None,
            overwrite_existing: true,
        }
    }

    pub fn builder(target_dir: impl Into<String>) -> RestoreOptionsBuilder {
        RestoreOptionsBuilder::new(target_dir)
    }
}

/// Builder for constructing `RestoreOptions`.
#[derive(Debug, Clone)]
pub struct RestoreOptionsBuilder {
    target_dir: String,
    encryption: EncryptionMode,
    filters: Vec<String>,
    overwrite_existing: bool,
}

impl RestoreOptionsBuilder {
    pub fn new(target_dir: impl Into<String>) -> Self {
        Self {
            target_dir: target_dir.into(),
            encryption: EncryptionMode::None,
            filters: Vec::new(),
            overwrite_existing: true,
        }
    }

    pub fn with_encryption(mut self, encryption: EncryptionMode) -> Self {
        self.encryption = encryption;
        self
    }

    pub fn with_filter(mut self, filter: impl Into<String>) -> Self {
        self.filters.push(filter.into());
        self
    }

    pub fn with_filters(mut self, filters: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for f in filters {
            self.filters.push(f.into());
        }
        self
    }

    pub fn overwrite_existing(mut self, overwrite: bool) -> Self {
        self.overwrite_existing = overwrite;
        self
    }

    pub fn build(self) -> RestoreOptions {
        RestoreOptions {
            target_dir: self.target_dir,
            encryption: self.encryption,
            filters: if self.filters.is_empty() {
                None
            } else {
                Some(self.filters)
            },
            overwrite_existing: self.overwrite_existing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_options_builder() {
        let opts = RestoreOptions::builder("/tmp/restore")
            .with_filter("DCIM")
            .with_filter("Documents/notes.txt")
            .with_encryption(EncryptionMode::Password("secret".to_string()))
            .overwrite_existing(false)
            .build();

        assert_eq!(opts.target_dir, "/tmp/restore");
        assert_eq!(opts.overwrite_existing, false);
        assert_eq!(
            opts.filters,
            Some(vec!["DCIM".to_string(), "Documents/notes.txt".to_string()])
        );
        assert_eq!(
            opts.encryption,
            EncryptionMode::Password("secret".to_string())
        );
    }
}
