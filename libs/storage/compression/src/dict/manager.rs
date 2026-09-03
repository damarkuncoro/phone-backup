use super::models::{CompressionDictionary, DictionaryId};
use crate::analysis::classifier::DataCategory;
use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe registry and manager for compression dictionaries.
#[derive(Debug, Clone, Default)]
pub struct DictionaryManager {
    dictionaries_by_id: HashMap<DictionaryId, Arc<CompressionDictionary>>,
    category_defaults: HashMap<DataCategory, DictionaryId>,
}

impl DictionaryManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a manager with pre-loaded Android-tailored dictionaries.
    pub fn with_android_defaults() -> Self {
        let mut manager = Self::new();

        // 1. Android XML Dictionary
        let xml_dict_data = b"<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<manifest xmlns:android=\"http://schemas.android.com/apk/res/android\"\npackage=\"\"\nandroid:name=\"\"\nandroid:value=\"\"\nandroid:exported=\"\"\nandroid:permission=\"\"\n<application\n<activity\n<service\n<receiver\n<provider\n<meta-data\n<uses-permission\n</manifest>";
        let xml_dict = CompressionDictionary::new(
            "android-xml-v1",
            DataCategory::Document,
            xml_dict_data.to_vec(),
        );

        // 2. Android JSON / Contacts / SMS dictionary
        let json_dict_data = b"{\"name\":\"\",\"phone\":\"\",\"type\":\"\",\"id\":\"\",\"timestamp\":0,\"address\":\"\",\"body\":\"\",\"read\":1,\"status\":0,\"contact_id\":0,\"date\":\"\",\"thread_id\":0,\"snippet\":\"\",\"messages\":[]}";
        let json_dict = CompressionDictionary::new(
            "android-json-v1",
            DataCategory::Document,
            json_dict_data.to_vec(),
        );

        // 3. SQLite header dictionary
        let sqlite_dict_data = b"SQLite format 3\x00CREATE TABLE \x00CREATE INDEX \x00PRIMARY KEY\x00AUTOINCREMENT\x00INTEGER\x00TEXT\x00BLOB\x00NOT NULL\x00DEFAULT \x00INSERT INTO \x00VALUES (";
        let sqlite_dict = CompressionDictionary::new(
            "android-sqlite-v1",
            DataCategory::Database,
            sqlite_dict_data.to_vec(),
        );

        manager.register(xml_dict);
        manager.register(json_dict.clone());
        manager.register(sqlite_dict.clone());

        manager.set_category_default(DataCategory::Document, json_dict.id);
        manager.set_category_default(DataCategory::Database, sqlite_dict.id);

        manager
    }

    /// Registers a dictionary into the manager.
    pub fn register(&mut self, dict: CompressionDictionary) {
        let id = dict.id.clone();
        self.dictionaries_by_id.insert(id, Arc::new(dict));
    }

    /// Sets the default dictionary for a data category.
    pub fn set_category_default(&mut self, category: DataCategory, id: impl Into<DictionaryId>) {
        self.category_defaults.insert(category, id.into());
    }

    /// Gets a dictionary by ID.
    pub fn get_by_id(&self, id: &DictionaryId) -> Option<Arc<CompressionDictionary>> {
        self.dictionaries_by_id.get(id).cloned()
    }

    /// Gets default dictionary for a category, if any.
    pub fn get_by_category(&self, category: DataCategory) -> Option<Arc<CompressionDictionary>> {
        let id = self.category_defaults.get(&category)?;
        self.get_by_id(id)
    }
}
