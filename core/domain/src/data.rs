use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub snapshot_id: Option<String>,
    pub source_id: Option<String>,
    pub display_name: String,
    pub notes: Option<String>,
    pub source: String,
    pub source_account: Option<String>,
    pub content_hash: Option<String>,
    pub metadata_json: Option<String>,
    pub names: Vec<ContactName>,
    pub phones: Vec<ContactPhone>,
    pub emails: Vec<ContactEmail>,
    pub addresses: Vec<ContactAddress>,
    pub organizations: Vec<ContactOrganization>,
    pub urls: Vec<ContactUrl>,
    pub events: Vec<ContactEvent>,
    pub photos: Vec<ContactPhoto>,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactName {
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPhone {
    pub raw_value: String,
    pub normalized_value: Option<String>,
    pub phone_type: Option<String>,
    pub label: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactEmail {
    pub value: String,
    pub email_type: Option<String>,
    pub label: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactAddress {
    pub formatted_address: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub address_type: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactOrganization {
    pub company_name: Option<String>,
    pub department: Option<String>,
    pub title: Option<String>,
    pub job_description: Option<String>,
    pub org_type: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactUrl {
    pub url: String,
    pub url_type: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactEvent {
    pub event_type: String,
    pub event_date: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPhoto {
    pub file_id: Option<String>,
    pub photo_hash: Option<String>,
    pub mime_type: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sms {
    pub address: String,
    pub body: String,
    pub date: DateTime<Utc>,
    pub type_code: u8, // 1: inbox, 2: sent, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLog {
    pub number: String,
    pub name: Option<String>,
    pub date: DateTime<Utc>,
    pub duration_seconds: u32,
    pub type_code: u8, // 1: incoming, 2: outgoing, 3: missed, etc.
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructuredData {
    Contacts(Vec<Contact>),
    SmsMessages(Vec<Sms>),
    CallLogs(Vec<CallLog>),
}
