use crate::model::Contact;

pub struct CsvFormatter;

impl CsvFormatter {
    /// Exports a list of contacts into standard Google Contacts / Excel compatible CSV.
    pub fn to_csv(contacts: &[Contact]) -> String {
        let mut out = String::new();
        // Header
        out.push_str("Name,Given Name,Family Name,Phone 1 - Value,Phone 1 - Type,E-mail 1 - Value,Organization,Notes\n");

        for c in contacts {
            let name = Self::escape_csv(&c.display_name);
            let given = Self::escape_csv(c.structured_name.given_name.as_deref().unwrap_or(""));
            let family = Self::escape_csv(c.structured_name.family_name.as_deref().unwrap_or(""));

            let (phone_val, phone_type) = if let Some(p) = c.primary_phone() {
                (Self::escape_csv(&p.raw), format!("{:?}", p.phone_type))
            } else {
                (String::new(), String::new())
            };

            let email_val = if let Some(e) = c.primary_email() {
                Self::escape_csv(&e.email)
            } else {
                String::new()
            };

            let org_val = if let Some(o) = &c.organization {
                Self::escape_csv(o.company.as_deref().unwrap_or(""))
            } else {
                String::new()
            };

            let notes_val = Self::escape_csv(c.notes.as_deref().unwrap_or(""));

            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                name, given, family, phone_val, phone_type, email_val, org_val, notes_val
            ));
        }

        out
    }

    fn escape_csv(val: &str) -> String {
        if val.contains(',') || val.contains('"') || val.contains('\n') {
            format!("\"{}\"", val.replace('"', "\"\""))
        } else {
            val.to_string()
        }
    }
}
