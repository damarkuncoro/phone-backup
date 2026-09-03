use crate::analysis::classifier::DataCategory;

#[cfg(feature = "derive")]
use serde::{Deserialize, Serialize};

/// Unique identifier for a compression dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub struct DictionaryId(pub String);

impl DictionaryId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T: Into<String>> From<T> for DictionaryId {
    fn from(s: T) -> Self {
        Self::new(s)
    }
}

/// A pre-trained compression dictionary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompressionDictionary {
    pub id: DictionaryId,
    pub category: DataCategory,
    pub data: Vec<u8>,
}

impl CompressionDictionary {
    pub fn new(id: impl Into<DictionaryId>, category: DataCategory, data: Vec<u8>) -> Self {
        Self {
            id: id.into(),
            category,
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
