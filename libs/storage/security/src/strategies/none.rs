use super::EncryptionStrategy;
use anyhow::Result;

#[derive(Default)]
pub struct NoEncryptionStrategy;

impl EncryptionStrategy for NoEncryptionStrategy {
    fn name(&self) -> &'static str {
        "none"
    }

    fn encrypt(&self, data: &[u8], _key: &[u8]) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    fn decrypt(&self, data: &[u8], _key: &[u8]) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }
}
