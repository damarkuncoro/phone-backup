use anyhow::Result;
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use rand::{RngCore, thread_rng};
use super::EncryptionStrategy;

#[derive(Default)]
pub struct AesGcmStrategy;

impl EncryptionStrategy for AesGcmStrategy {
    fn name(&self) -> &'static str {
        "aes-256-gcm"
    }

    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|_| anyhow::anyhow!("Invalid key length for AES"))?;

        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, Payload { msg: data, aad: b"" })
            .map_err(|e| anyhow::anyhow!("AES encryption error: {}", e))?;

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 12 {
            anyhow::bail!("Invalid encrypted data: too short for AES");
        }

        let nonce_bytes = &data[0..12];
        let ciphertext = &data[12..];
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|_| anyhow::anyhow!("Invalid key length for AES"))?;

        let plaintext = cipher
            .decrypt(nonce, Payload { msg: ciphertext, aad: b"" })
            .map_err(|e| anyhow::anyhow!("AES decryption error: {}", e))?;

        Ok(plaintext)
    }
}
