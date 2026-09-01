use anyhow::Result;

pub mod chacha;
pub mod aes;
pub mod none;

pub trait EncryptionStrategy: Send + Sync {
    fn name(&self) -> &'static str;
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>>;
}
