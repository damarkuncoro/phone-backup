use domain::{AppInfo, AppId, DeviceId};
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

    pub fn to_contact_name(row: &Row) -> rusqlite::Result<domain::ContactName> {
        Ok(domain::ContactName {
            display_name: row.get(2)?,
            given_name: row.get(3)?,
            middle_name: row.get(4)?,
            family_name: row.get(5)?,
            prefix: row.get(6)?,
            suffix: row.get(7)?,
        })
    }

    pub fn to_contact_phone(row: &Row) -> rusqlite::Result<domain::ContactPhone> {
        Ok(domain::ContactPhone {
            raw_value: row.get(2)?,
            normalized_value: row.get(3)?,
            phone_type: row.get(4)?,
            label: row.get(5)?,
            is_primary: row.get::<_, i32>(6)? == 1,
        })
    }

    pub fn to_contact_email(row: &Row) -> rusqlite::Result<domain::ContactEmail> {
        Ok(domain::ContactEmail {
            value: row.get(2)?,
            email_type: row.get(3)?,
            label: row.get(4)?,
            is_primary: row.get::<_, i32>(5)? == 1,
        })
    }

    pub fn to_contact_address(row: &Row) -> rusqlite::Result<domain::ContactAddress> {
        Ok(domain::ContactAddress {
            formatted_address: row.get(2)?,
            street: row.get(3)?,
            city: row.get(4)?,
            region: row.get(5)?,
            postal_code: row.get(6)?,
            country: row.get(7)?,
            country_code: row.get(8)?,
            address_type: row.get(9)?,
            label: row.get(10)?,
        })
    }

    pub fn to_contact_organization(row: &Row) -> rusqlite::Result<domain::ContactOrganization> {
        Ok(domain::ContactOrganization {
            company_name: row.get(2)?,
            department: row.get(3)?,
            title: row.get(4)?,
            job_description: row.get(5)?,
            org_type: row.get(6)?,
            label: row.get(7)?,
        })
    }

    pub fn to_contact_url(row: &Row) -> rusqlite::Result<domain::ContactUrl> {
        Ok(domain::ContactUrl {
            url: row.get(2)?,
            url_type: row.get(3)?,
            label: row.get(4)?,
        })
    }

    pub fn to_contact_event(row: &Row) -> rusqlite::Result<domain::ContactEvent> {
        Ok(domain::ContactEvent {
            event_type: row.get(2)?,
            event_date: row.get(3)?,
            label: row.get(4)?,
        })
    }

    pub fn to_contact_photo(row: &Row) -> rusqlite::Result<domain::ContactPhoto> {
        Ok(domain::ContactPhoto {
            file_id: row.get(2)?,
            photo_hash: row.get(3)?,
            mime_type: row.get(4)?,
            is_primary: row.get::<_, i32>(5)? == 1,
        })
    }
}
