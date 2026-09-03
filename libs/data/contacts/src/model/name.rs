use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StructuredName {
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl StructuredName {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_given(mut self, given: impl Into<String>) -> Self {
        self.given_name = Some(given.into());
        self
    }

    pub fn with_family(mut self, family: impl Into<String>) -> Self {
        self.family_name = Some(family.into());
        self
    }

    pub fn with_middle(mut self, middle: impl Into<String>) -> Self {
        self.middle_name = Some(middle.into());
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn full_name(&self) -> String {
        let mut parts = Vec::new();
        if let Some(p) = &self.prefix {
            parts.push(p.as_str());
        }
        if let Some(g) = &self.given_name {
            parts.push(g.as_str());
        }
        if let Some(m) = &self.middle_name {
            parts.push(m.as_str());
        }
        if let Some(f) = &self.family_name {
            parts.push(f.as_str());
        }
        if let Some(s) = &self.suffix {
            parts.push(s.as_str());
        }
        parts.join(" ")
    }
}
