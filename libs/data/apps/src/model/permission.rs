use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionProtection {
    Normal,
    Dangerous,
    Signature,
    Privileged,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionEntry {
    pub name: String,
    pub protection_level: PermissionProtection,
    pub is_critical_privacy: bool,
}

impl PermissionEntry {
    pub fn new(name: impl Into<String>) -> Self {
        let name_str = name.into();
        let protection_level = Self::classify_protection(&name_str);
        let is_critical_privacy = Self::is_privacy_sensitive(&name_str);

        Self {
            name: name_str,
            protection_level,
            is_critical_privacy,
        }
    }

    fn classify_protection(name: &str) -> PermissionProtection {
        let dangerous_list = [
            "android.permission.READ_CONTACTS",
            "android.permission.WRITE_CONTACTS",
            "android.permission.READ_SMS",
            "android.permission.SEND_SMS",
            "android.permission.RECEIVE_SMS",
            "android.permission.ACCESS_FINE_LOCATION",
            "android.permission.ACCESS_COARSE_LOCATION",
            "android.permission.CAMERA",
            "android.permission.RECORD_AUDIO",
            "android.permission.READ_CALL_LOG",
            "android.permission.WRITE_CALL_LOG",
            "android.permission.READ_EXTERNAL_STORAGE",
            "android.permission.WRITE_EXTERNAL_STORAGE",
            "android.permission.MANAGE_EXTERNAL_STORAGE",
        ];

        if dangerous_list.contains(&name) {
            PermissionProtection::Dangerous
        } else if name.contains("BIND_") || name.contains("SYSTEM_") {
            PermissionProtection::Privileged
        } else {
            PermissionProtection::Normal
        }
    }

    fn is_privacy_sensitive(name: &str) -> bool {
        name.contains("CONTACTS")
            || name.contains("SMS")
            || name.contains("LOCATION")
            || name.contains("CAMERA")
            || name.contains("RECORD_AUDIO")
            || name.contains("CALL_LOG")
    }
}
