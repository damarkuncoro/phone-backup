use domain::Contact;

pub fn export_to_vcard(contacts: &[Contact]) -> String {
    let mut vcard_out = String::new();

    for contact in contacts {
        vcard_out.push_str("BEGIN:VCARD\r\n");
        vcard_out.push_str("VERSION:4.0\r\n");

        // N: Family;Given;Middle;Prefix;Suffix
        if let Some(name) = contact.names.first() {
            let family = name.family_name.as_deref().unwrap_or("");
            let given = name.given_name.as_deref().unwrap_or("");
            let middle = name.middle_name.as_deref().unwrap_or("");
            let prefix = name.prefix.as_deref().unwrap_or("");
            let suffix = name.suffix.as_deref().unwrap_or("");
            vcard_out.push_str(&format!(
                "N:{};{};{};{};{}\r\n",
                family, given, middle, prefix, suffix
            ));
        }

        vcard_out.push_str(&format!("FN:{}\r\n", contact.display_name));

        // TEL
        for phone in &contact.phones {
            let ptype = phone.phone_type.as_deref().unwrap_or("cell").to_uppercase();
            vcard_out.push_str(&format!("TEL;TYPE={}:{}\r\n", ptype, phone.raw_value));
        }

        // EMAIL
        for email in &contact.emails {
            let etype = email
                .email_type
                .as_deref()
                .unwrap_or("INTERNET")
                .to_uppercase();
            vcard_out.push_str(&format!("EMAIL;TYPE={}:{}\r\n", etype, email.value));
        }

        // ADR: ;;Street;City;Region;PostalCode;Country
        for adr in &contact.addresses {
            let street = adr.street.as_deref().unwrap_or("");
            let city = adr.city.as_deref().unwrap_or("");
            let region = adr.region.as_deref().unwrap_or("");
            let postal = adr.postal_code.as_deref().unwrap_or("");
            let country = adr.country.as_deref().unwrap_or("");
            vcard_out.push_str(&format!(
                "ADR:;;{};{};{};{};{}\r\n",
                street, city, region, postal, country
            ));
        }

        // ORG & TITLE
        for org in &contact.organizations {
            if let Some(comp) = &org.company_name {
                vcard_out.push_str(&format!("ORG:{}\r\n", comp));
            }
            if let Some(title) = &org.title {
                vcard_out.push_str(&format!("TITLE:{}\r\n", title));
            }
        }

        // URL
        for url in &contact.urls {
            vcard_out.push_str(&format!("URL:{}\r\n", url.url));
        }

        // NOTE
        if let Some(notes) = &contact.notes {
            let escaped = notes.replace('\n', "\\n");
            vcard_out.push_str(&format!("NOTE:{}\r\n", escaped));
        }

        vcard_out.push_str("END:VCARD\r\n\r\n");
    }

    vcard_out
}
