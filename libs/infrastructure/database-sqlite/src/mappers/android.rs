use domain::{AppId, AppInfo, DeviceId};
use rusqlite::Row;

pub struct AndroidMapper;

impl AndroidMapper {
    pub fn to_app(row: &Row) -> rusqlite::Result<AppInfo> {
        Ok(AppInfo {
            id: AppId(row.get(0)?),
            device_id: DeviceId(row.get(1)?),
            package_name: row.get(2)?,
            version_name: row.get(3)?,
            version_code: row.get(4)?,
            installer: row.get(5)?,
            app_name: row.get(6)?,
        })
    }
}
