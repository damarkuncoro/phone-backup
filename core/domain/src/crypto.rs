use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionMode {
    None,
    Password(String),
    PublicKey(String),
}

impl EncryptionMode {
    pub fn is_encrypted(&self) -> bool {
        !matches!(self, EncryptionMode::None)
    }
}
