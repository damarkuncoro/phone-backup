use crate::model::fields::{EmailAddress, EmailType, Organization, PhoneNumber, PhoneType, PostalAddress};
use crate::model::name::StructuredName;
use crate::model::Contact;
use crate::ContactBook;

pub struct ContactBuilder {
    contact: Contact,
}

impl ContactBuilder {
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            contact: Contact::new(display_name),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.contact.id = Some(id.into());
        self
    }

    pub fn with_account(mut self, account: impl Into<String>) -> Self {
        self.contact.source_account = Some(account.into());
        self
    }

    pub fn with_structured_name(mut self, name: StructuredName) -> Self {
        self.contact.structured_name = name;
        self
    }

    pub fn add_phone(mut self, raw: impl Into<String>, phone_type: PhoneType) -> Self {
        self.contact.phone_numbers.push(PhoneNumber::new(raw, phone_type));
        self
    }

    pub fn add_email(mut self, email: impl Into<String>, email_type: EmailType) -> Self {
        self.contact.emails.push(EmailAddress::new(email, email_type));
        self
    }

    pub fn add_address(mut self, addr: PostalAddress) -> Self {
        self.contact.addresses.push(addr);
        self
    }

    pub fn with_organization(mut self, company: impl Into<String>, title: Option<&str>) -> Self {
        self.contact.organization = Some(Organization {
            company: Some(company.into()),
            title: title.map(|t| t.to_string()),
            department: None,
        });
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.contact.notes = Some(notes.into());
        self
    }

    pub fn with_birthday(mut self, birthday: impl Into<String>) -> Self {
        self.contact.birthday = Some(birthday.into());
        self
    }

    pub fn starred(mut self, is_starred: bool) -> Self {
        self.contact.starred = is_starred;
        self
    }

    pub fn build(self) -> Contact {
        self.contact
    }
}

pub struct ContactBookBuilder {
    contacts: Vec<Contact>,
    default_country_code: String,
}

impl ContactBookBuilder {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
            default_country_code: "+62".to_string(),
        }
    }

    pub fn with_default_country_code(mut self, country_code: impl Into<String>) -> Self {
        self.default_country_code = country_code.into();
        self
    }

    pub fn add_contact(mut self, contact: Contact) -> Self {
        self.contacts.push(contact);
        self
    }

    pub fn add_contacts(mut self, contacts: impl IntoIterator<Item = Contact>) -> Self {
        self.contacts.extend(contacts);
        self
    }

    pub fn build(self) -> ContactBook {
        ContactBook::new(self.contacts, self.default_country_code)
    }
}

impl Default for ContactBookBuilder {
    fn default() -> Self {
        Self::new()
    }
}
