use crate::parsers::common::ParserUtils;
use domain::{
    Contact, ContactAddress, ContactEmail, ContactEvent, ContactName, ContactOrganization,
    ContactPhone, ContactUrl, DeviceId,
};
use sha2::{Digest, Sha256};

pub struct ContactParser;

impl ContactParser {
    pub fn parse(_device_id: &DeviceId, output: &str) -> Vec<Contact> {
        let mut contacts_map = std::collections::HashMap::new();

        for line in output.lines() {
            let contact_id =
                ParserUtils::extract_value(line, "contact_id").unwrap_or_else(|| "0".to_string());
            let display_name = ParserUtils::extract_value(line, "display_name")
                .unwrap_or_else(|| "Unknown".to_string());
            let mimetype = ParserUtils::extract_value(line, "mimetype").unwrap_or_default();
            let account_name = ParserUtils::extract_value(line, "account_name");

            let data1 = ParserUtils::extract_value(line, "data1");
            let data2 = ParserUtils::extract_value(line, "data2");
            let data3 = ParserUtils::extract_value(line, "data3");
            let data4 = ParserUtils::extract_value(line, "data4");
            let data5 = ParserUtils::extract_value(line, "data5");
            let data7 = ParserUtils::extract_value(line, "data7");
            let data8 = ParserUtils::extract_value(line, "data8");
            let data9 = ParserUtils::extract_value(line, "data9");

            let contact = contacts_map.entry(contact_id.clone()).or_insert(Contact {
                id: uuid::Uuid::new_v4().to_string(),
                snapshot_id: None,
                source_id: Some(contact_id),
                display_name,
                notes: None,
                source: "android".to_string(),
                source_account: account_name,
                content_hash: None,
                metadata_json: None,
                names: vec![],
                phones: vec![],
                emails: vec![],
                addresses: vec![],
                organizations: vec![],
                urls: vec![],
                events: vec![],
                photos: vec![],
                labels: vec![],
            });

            if mimetype.contains("name") {
                contact.names.push(ContactName {
                    display_name: data1,
                    given_name: data2,
                    family_name: data3,
                    prefix: data4,
                    middle_name: data5,
                    suffix: None,
                });
            } else if mimetype.contains("phone") {
                contact.phones.push(ContactPhone {
                    raw_value: data1.unwrap_or_default(),
                    normalized_value: data4,
                    phone_type: data2,
                    label: data3,
                    is_primary: false,
                });
            } else if mimetype.contains("email") {
                contact.emails.push(ContactEmail {
                    value: data1.unwrap_or_default(),
                    email_type: data2,
                    label: data3,
                    is_primary: false,
                });
            } else if mimetype.contains("postal-address") {
                contact.addresses.push(ContactAddress {
                    formatted_address: data1,
                    address_type: data2,
                    label: data3,
                    street: data4,
                    postal_code: data9,
                    city: data7,
                    region: data8,
                    country: None,
                    country_code: None,
                });
            } else if mimetype.contains("organization") {
                contact.organizations.push(ContactOrganization {
                    company_name: data1,
                    org_type: data2,
                    label: data3,
                    title: data4,
                    department: data5,
                    job_description: None,
                });
            } else if mimetype.contains("note") {
                contact.notes = data1;
            } else if mimetype.contains("website") {
                contact.urls.push(ContactUrl {
                    url: data1.unwrap_or_default(),
                    url_type: data2,
                    label: data3,
                });
            } else if mimetype.contains("event") {
                contact.events.push(ContactEvent {
                    event_date: data1.unwrap_or_default(),
                    event_type: data2.unwrap_or_else(|| "custom".to_string()),
                    label: data3,
                });
            }
        }

        // Calculate content hashes for deduplication
        for contact in contacts_map.values_mut() {
            let json = serde_json::to_string(&contact).unwrap_or_default();
            contact.content_hash = Some(
                Sha256::digest(json.as_bytes())
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect(),
            );
        }

        contacts_map.into_values().collect()
    }
}
