use crate::model::{AppManifest, AppPackage, AppType, PermissionEntry};

pub struct AppPackageBuilder {
    pkg: AppPackage,
}

impl AppPackageBuilder {
    pub fn new(package_name: impl Into<String>, app_name: impl Into<String>) -> Self {
        Self {
            pkg: AppPackage::new(package_name, app_name, 1, "1.0"),
        }
    }

    pub fn with_version(mut self, version_code: u64, version_name: impl Into<String>) -> Self {
        self.pkg.version_code = version_code;
        self.pkg.version_name = version_name.into();
        self
    }

    pub fn with_app_type(mut self, app_type: AppType) -> Self {
        self.pkg.app_type = app_type;
        self
    }

    pub fn with_sizes(mut self, apk_size_bytes: u64, data_size_bytes: u64) -> Self {
        self.pkg.apk_size_bytes = apk_size_bytes;
        self.pkg.data_size_bytes = data_size_bytes;
        self
    }

    pub fn with_splits(mut self, splits_count: usize) -> Self {
        self.pkg.is_split = splits_count > 1;
        self.pkg.splits_count = splits_count;
        self
    }

    pub fn build(self) -> AppPackage {
        self.pkg
    }
}

pub struct AppManifestBuilder {
    manifest: AppManifest,
}

impl AppManifestBuilder {
    pub fn new(package_name: impl Into<String>, min_sdk: u32, target_sdk: u32) -> Self {
        Self {
            manifest: AppManifest::new(package_name, 1, "1.0", min_sdk, target_sdk),
        }
    }

    pub fn with_version(mut self, version_code: u64, version_name: impl Into<String>) -> Self {
        self.manifest.version_code = version_code;
        self.manifest.version_name = version_name.into();
        self
    }

    pub fn add_permission(mut self, permission_name: impl Into<String>) -> Self {
        self.manifest.permissions.push(PermissionEntry::new(permission_name));
        self
    }

    pub fn with_debuggable(mut self, is_debuggable: bool) -> Self {
        self.manifest.is_debuggable = is_debuggable;
        self
    }

    pub fn with_allow_backup(mut self, allow: bool) -> Self {
        self.manifest.allow_backup = allow;
        self
    }

    pub fn build(self) -> AppManifest {
        self.manifest
    }
}
