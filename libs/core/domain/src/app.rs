use crate::DeviceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub id: AppId,
    pub device_id: DeviceId,
    pub package_name: String,
    pub version_name: String,
    pub version_code: u32,
    pub installer: Option<String>,
    pub app_name: String,
}
