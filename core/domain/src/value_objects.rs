use serde::{Deserialize, Serialize};
use std::fmt;

/// Type-safe SHA-256 Checksum Value Object enforcing 64-hex-character invariants.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Checksum(String);

impl Checksum {
    pub fn new(hash: impl Into<String>) -> Result<Self, crate::DomainError> {
        let h = hash.into().trim().to_lowercase();
        if h.len() == 64 && h.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(h))
        } else {
            Err(crate::DomainError::ValidationError(format!(
                "Invalid SHA-256 checksum format: '{}'",
                h
            )))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type-safe Storage Size Value Object with human-readable formatting and arithmetic helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct StorageSize(u64);

impl StorageSize {
    pub fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    pub fn bytes(&self) -> u64 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn format_human_readable(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if self.0 >= TB {
            format!("{:.2} TB", self.0 as f64 / TB as f64)
        } else if self.0 >= GB {
            format!("{:.2} GB", self.0 as f64 / GB as f64)
        } else if self.0 >= MB {
            format!("{:.2} MB", self.0 as f64 / MB as f64)
        } else if self.0 >= KB {
            format!("{:.2} KB", self.0 as f64 / KB as f64)
        } else {
            format!("{} B", self.0)
        }
    }
}

impl fmt::Display for StorageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_human_readable())
    }
}

impl std::ops::Add for StorageSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

/// Type-safe Android Device Storage Path Value Object enforcing path normalization and safety.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DevicePath(String);

impl DevicePath {
    pub fn new(path: impl Into<String>) -> Result<Self, crate::DomainError> {
        let raw = path.into();
        let normalized = raw.replace('\\', "/");

        // Prevent path traversal attacks (e.g., ../)
        if normalized.contains("../") || normalized.contains("/..") || normalized == ".." {
            return Err(crate::DomainError::ValidationError(format!(
                "Security Risk: Path traversal detected in DevicePath: '{}'",
                raw
            )));
        }

        let cleaned = if !normalized.starts_with('/') {
            format!("/{}", normalized)
        } else {
            normalized
        };

        Ok(Self(cleaned))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn extension(&self) -> Option<&str> {
        std::path::Path::new(&self.0)
            .extension()
            .and_then(|ext| ext.to_str())
    }

    pub fn parent(&self) -> Option<Self> {
        std::path::Path::new(&self.0)
            .parent()
            .and_then(|p| p.to_str())
            .and_then(|p| Self::new(p).ok())
    }
}

impl fmt::Display for DevicePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
