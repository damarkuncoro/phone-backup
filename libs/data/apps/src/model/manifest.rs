use super::permission::PermissionEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkVersion {
    pub min_sdk: u32,
    pub target_sdk: u32,
    pub compile_sdk: Option<u32>,
}

impl SdkVersion {
    pub fn new(min_sdk: u32, target_sdk: u32) -> Self {
        Self {
            min_sdk,
            target_sdk,
            compile_sdk: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppManifest {
    pub package_name: String,
    pub version_code: u64,
    pub version_name: String,
    pub sdk: SdkVersion,
    pub permissions: Vec<PermissionEntry>,
    pub app_label: Option<String>,
    pub main_activity: Option<String>,
    pub is_debuggable: bool,
    pub allow_backup: bool,
}

impl AppManifest {
    pub fn new(package_name: impl Into<String>, version_code: u64, version_name: impl Into<String>, min_sdk: u32, target_sdk: u32) -> Self {
        Self {
            package_name: package_name.into(),
            version_code,
            version_name: version_name.into(),
            sdk: SdkVersion::new(min_sdk, target_sdk),
            permissions: Vec::new(),
            app_label: None,
            main_activity: None,
            is_debuggable: false,
            allow_backup: true,
        }
    }
}
