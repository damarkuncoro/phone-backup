use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApkSplitType {
    Base,
    ConfigAbi,
    ConfigDensity,
    ConfigLocale,
    DynamicFeature,
    Unknown,
}

impl ApkSplitType {
    pub fn from_filename(name: &str) -> Self {
        if name == "base.apk" || !name.contains("split_") {
            Self::Base
        } else if name.contains("arm64") || name.contains("armeabi") || name.contains("x86") {
            Self::ConfigAbi
        } else if name.contains("hdpi") || name.contains("xxhdpi") || name.contains("xxxhdpi") || name.contains("ldpi") {
            Self::ConfigDensity
        } else if name.contains("split_config.") && name.len() <= 20 {
            Self::ConfigLocale
        } else {
            Self::DynamicFeature
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitApkFile {
    pub filename: String,
    pub split_type: ApkSplitType,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitApkBundle {
    pub package_name: String,
    pub version_code: u64,
    pub files: Vec<SplitApkFile>,
}

impl SplitApkBundle {
    pub fn new(package_name: impl Into<String>, version_code: u64) -> Self {
        Self {
            package_name: package_name.into(),
            version_code,
            files: Vec::new(),
        }
    }

    pub fn add_split(&mut self, filename: impl Into<String>, size_bytes: u64) {
        let fn_str = filename.into();
        let split_type = ApkSplitType::from_filename(&fn_str);
        self.files.push(SplitApkFile {
            filename: fn_str,
            split_type,
            size_bytes,
        });
    }

    pub fn total_size(&self) -> u64 {
        self.files.iter().map(|f| f.size_bytes).sum()
    }
}
