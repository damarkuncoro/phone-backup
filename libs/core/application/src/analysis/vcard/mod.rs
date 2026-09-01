pub mod export;
pub mod import;

use anyhow::Result;
use domain::Contact;

pub struct VCardEngine;

impl VCardEngine {
    pub fn export_to_vcard(contacts: &[Contact]) -> String {
        export::export_to_vcard(contacts)
    }

    pub fn import_from_vcard(vcard_data: &str) -> Result<Vec<Contact>> {
        import::import_from_vcard(vcard_data)
    }
}
