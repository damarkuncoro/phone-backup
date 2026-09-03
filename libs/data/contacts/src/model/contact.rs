use super::fields::{EmailAddress, Organization, PhoneNumber, PostalAddress, SocialProfile};
use super::name::StructuredName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactPhoto {
    pub mime_type: String,
    pub data: Vec<u8>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Contact {
    pub id: Option<String>,
    pub source_account: Option<String>,
    pub display_name: String,
    pub structured_name: StructuredName,
    pub phone_numbers: Vec<PhoneNumber>,
    pub emails: Vec<EmailAddress>,
    pub addresses: Vec<PostalAddress>,
    pub organization: Option<Organization>,
    pub social_profiles: Vec<SocialProfile>,
    pub notes: Option<String>,
    pub birthday: Option<String>,
    pub starred: bool,
    pub photos: Vec<ContactPhoto>,
}

impl Contact {
    pub fn new(display_name: impl Into<String>) -> Self {
        let name = display_name.into();
        Self {
            id: None,
            source_account: None,
            display_name: name,
            structured_name: StructuredName::default(),
            phone_numbers: Vec::new(),
            emails: Vec::new(),
            addresses: Vec::new(),
            organization: None,
            social_profiles: Vec::new(),
            notes: None,
            birthday: None,
            starred: false,
            photos: Vec::new(),
        }
    }

    pub fn primary_phone(&self) -> Option<&PhoneNumber> {
        self.phone_numbers
            .iter()
            .find(|p| p.is_primary)
            .or_else(|| self.phone_numbers.first())
    }

    pub fn primary_email(&self) -> Option<&EmailAddress> {
        self.emails
            .iter()
            .find(|e| e.is_primary)
            .or_else(|| self.emails.first())
    }

    pub fn is_empty(&self) -> bool {
        self.display_name.trim().is_empty()
            && self.phone_numbers.is_empty()
            && self.emails.is_empty()
    }
}
