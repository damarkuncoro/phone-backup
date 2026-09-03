use crate::model::ContactPhoto;
use base64::Engine;

pub struct PhotoHandler;

impl PhotoHandler {
    /// Decodes base64 photo data into ContactPhoto struct.
    pub fn decode_base64(raw_base64: &str, mime_type: Option<&str>) -> Option<ContactPhoto> {
        let cleaned: String = raw_base64.chars().filter(|c| !c.is_whitespace()).collect();
        let engine = base64::engine::general_purpose::STANDARD;
        match engine.decode(cleaned.as_bytes()) {
            Ok(data) => Some(ContactPhoto {
                mime_type: mime_type.unwrap_or("image/jpeg").to_string(),
                data,
                is_primary: true,
            }),
            Err(_) => None,
        }
    }

    /// Encodes ContactPhoto data to base64 string.
    pub fn encode_base64(photo: &ContactPhoto) -> String {
        let engine = base64::engine::general_purpose::STANDARD;
        engine.encode(&photo.data)
    }
}
