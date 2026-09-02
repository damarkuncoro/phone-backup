use super::EncryptionStrategy;
use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
use rand::{thread_rng, RngCore};

#[derive(Default)]
pub struct ChaChaStrategy;

impl EncryptionStrategy for ChaChaStrategy {
    fn name(&self) -> &'static str {
        "xchacha20-poly1305"
    }

    fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new_from_slice(key)
            .map_err(|_| anyhow::anyhow!("Invalid key length for ChaCha"))?;

        let mut nonce_bytes = [0u8; 24];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = XNonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(
                nonce,
                Payload {
                    msg: data,
                    aad: b"",
                },
            )
            .map_err(|e| anyhow::anyhow!("ChaCha encryption error: {}", e))?;

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 24 {
            anyhow::bail!("Invalid encrypted data: too short for ChaCha");
        }

        let nonce_bytes = &data[0..24];
        let ciphertext = &data[24..];
        let nonce = XNonce::from_slice(nonce_bytes);

        let cipher = XChaCha20Poly1305::new_from_slice(key)
            .map_err(|_| anyhow::anyhow!("Invalid key length for ChaCha"))?;

        let plaintext = cipher
            .decrypt(
                nonce,
                Payload {
                    msg: ciphertext,
                    aad: b"",
                },
            )
            .map_err(|e| anyhow::anyhow!("ChaCha decryption error: {}", e))?;

        Ok(plaintext)
    }
}
