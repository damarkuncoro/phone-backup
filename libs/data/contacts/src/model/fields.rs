use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhoneType {
    Mobile,
    Home,
    Work,
    Main,
    Fax,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub raw: String,
    pub normalized_e164: Option<String>,
    pub phone_type: PhoneType,
    pub is_primary: bool,
}

impl PhoneNumber {
    pub fn new(raw: impl Into<String>, phone_type: PhoneType) -> Self {
        Self {
            raw: raw.into(),
            normalized_e164: None,
            phone_type,
            is_primary: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmailType {
    Personal,
    Work,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailAddress {
    pub email: String,
    pub email_type: EmailType,
    pub is_primary: bool,
}

impl EmailAddress {
    pub fn new(email: impl Into<String>, email_type: EmailType) -> Self {
        Self {
            email: email.into(),
            email_type,
            is_primary: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PostalAddress {
    pub street: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocialProfile {
    pub service: String,
    pub handle: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organization {
    pub company: Option<String>,
    pub title: Option<String>,
    pub department: Option<String>,
}
