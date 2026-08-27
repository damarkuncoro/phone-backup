use crate::compression::CompressionEngine;

pub struct ObjectStoreKey;

impl ObjectStoreKey {
    pub fn compute_object_id(hash: &str, mime_type: Option<&str>, is_encrypted: bool) -> String {
        let mut object_id = if let Some(mime) = mime_type {
            if CompressionEngine::should_compress(mime) {
                format!("{}.zst", hash)
            } else {
                hash.to_string()
            }
        } else {
            hash.to_string()
        };

        if is_encrypted {
            object_id = format!("{}.enc", object_id);
        }

        object_id
    }

    pub fn compute_object_path(hash: &str, object_id: &str) -> String {
        format!("objects/{}/{}/{}", &hash[0..2], &hash[2..4], object_id)
    }
}
