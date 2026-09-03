use domain::Contact;

pub const FULL_CONTACT_SELECT: &str = r#"
    SELECT
        c.id, c.source_id, c.display_name, c.notes, c.source, c.source_account, c.content_hash, c.metadata_json,
        (SELECT COALESCE(json_group_array(json_object(
            'display_name', display_name, 'given_name', given_name, 'middle_name', middle_name,
            'family_name', family_name, 'prefix', prefix, 'suffix', suffix
        )), '[]') FROM contact_names WHERE contact_id = c.id) as names_json,
        (SELECT COALESCE(json_group_array(json_object(
            'raw_value', raw_value, 'normalized_value', normalized_value,
            'phone_type', type, 'label', label,
            'is_primary', CASE WHEN is_primary != 0 THEN json('true') ELSE json('false') END
        )), '[]') FROM contact_phones WHERE contact_id = c.id) as phones_json,
        (SELECT COALESCE(json_group_array(json_object(
            'value', value, 'email_type', type, 'label', label,
            'is_primary', CASE WHEN is_primary != 0 THEN json('true') ELSE json('false') END
        )), '[]') FROM contact_emails WHERE contact_id = c.id) as emails_json,
        (SELECT COALESCE(json_group_array(json_object(
            'formatted_address', formatted_address, 'street', street, 'city', city,
            'region', region, 'postal_code', postal_code, 'country', country,
            'country_code', country_code, 'address_type', type, 'label', label
        )), '[]') FROM contact_addresses WHERE contact_id = c.id) as addresses_json,
        (SELECT COALESCE(json_group_array(json_object(
            'company_name', company_name, 'department', department, 'title', title,
            'job_description', job_description, 'org_type', type, 'label', label
        )), '[]') FROM contact_organizations WHERE contact_id = c.id) as organizations_json,
        (SELECT COALESCE(json_group_array(json_object(
            'url', url, 'url_type', type, 'label', label
        )), '[]') FROM contact_urls WHERE contact_id = c.id) as urls_json,
        (SELECT COALESCE(json_group_array(json_object(
            'event_type', event_type, 'event_date', event_date, 'label', label
        )), '[]') FROM contact_events WHERE contact_id = c.id) as events_json,
        (SELECT COALESCE(json_group_array(json_object(
            'file_id', file_id, 'photo_hash', photo_hash, 'mime_type', mime_type,
            'is_primary', CASE WHEN is_primary != 0 THEN json('true') ELSE json('false') END
        )), '[]') FROM contact_photos WHERE contact_id = c.id) as photos_json,
        (SELECT COALESCE(json_group_array(cl.name), '[]')
         FROM contact_labels cl JOIN contact_label_members clm ON cl.id = clm.label_id
         WHERE clm.contact_id = c.id) as labels_json
    FROM contact_objects c
"#;

pub fn map_row_to_contact(row: &rusqlite::Row) -> rusqlite::Result<Contact> {
    let id: String = row.get(0)?;
    let source_id: Option<String> = row.get(1)?;
    let display_name: String = row.get(2)?;
    let notes: Option<String> = row.get(3)?;
    let source: String = row.get(4)?;
    let source_account: Option<String> = row.get(5)?;
    let content_hash: Option<String> = row.get(6)?;
    let metadata_json: Option<String> = row.get(7)?;

    let names_json: String = row.get("names_json")?;
    let phones_json: String = row.get("phones_json")?;
    let emails_json: String = row.get("emails_json")?;
    let addresses_json: String = row.get("addresses_json")?;
    let organizations_json: String = row.get("organizations_json")?;
    let urls_json: String = row.get("urls_json")?;
    let events_json: String = row.get("events_json")?;
    let photos_json: String = row.get("photos_json")?;
    let labels_json: String = row.get("labels_json")?;

    Ok(Contact {
        id,
        snapshot_id: None,
        source_id,
        display_name,
        notes,
        source,
        source_account,
        content_hash,
        metadata_json,
        names: serde_json::from_str(&names_json).unwrap_or_default(),
        phones: serde_json::from_str(&phones_json).unwrap_or_default(),
        emails: serde_json::from_str(&emails_json).unwrap_or_default(),
        addresses: serde_json::from_str(&addresses_json).unwrap_or_default(),
        organizations: serde_json::from_str(&organizations_json).unwrap_or_default(),
        urls: serde_json::from_str(&urls_json).unwrap_or_default(),
        events: serde_json::from_str(&events_json).unwrap_or_default(),
        photos: serde_json::from_str(&photos_json).unwrap_or_default(),
        labels: serde_json::from_str(&labels_json).unwrap_or_default(),
    })
}
