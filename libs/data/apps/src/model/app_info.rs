use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppType {
    User,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Architecture {
    Arm64V8a,
    ArmeabiV7a,
    X86_64,
    X86,
    Universal,
    Unknown,
}

impl Architecture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Arm64V8a => "arm64-v8a",
            Self::ArmeabiV7a => "armeabi-v7a",
            Self::X86_64 => "x86_64",
            Self::X86 => "x86",
            Self::Universal => "universal",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPackage {
    pub package_name: String,
    pub app_name: String,
    pub version_code: u64,
    pub version_name: String,
    pub app_type: AppType,
    pub apk_size_bytes: u64,
    pub data_size_bytes: u64,
    pub is_split: bool,
    pub splits_count: usize,
}

impl AppPackage {
    pub fn new(package_name: impl Into<String>, app_name: impl Into<String>, version_code: u64, version_name: impl Into<String>) -> Self {
        Self {
            package_name: package_name.into(),
            app_name: app_name.into(),
            version_code,
            version_name: version_name.into(),
            app_type: AppType::User,
            apk_size_bytes: 0,
            data_size_bytes: 0,
            is_split: false,
            splits_count: 1,
        }
    }
}
