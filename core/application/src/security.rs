use anyhow::Result;
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use argon2::Argon2;
use rand::{RngCore, thread_rng};

pub struct EncryptionEngine;

impl EncryptionEngine {
    pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
        let salt = b"static_salt_for_demo"; // In real app, use random salt and store it
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| anyhow::anyhow!("KDF error: {}", e))?;

        let cipher = Aes256Gcm::new_from_slice(&key)?;
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, Payload { msg: data, aad: b"" })
            .map_err(|e| anyhow::anyhow!("Encryption error: {}", e))?;

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    pub fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
        if data.len() < 12 {
            anyhow::bail!("Invalid encrypted data: too short");
        }

        let salt = b"static_salt_for_demo";
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| anyhow::anyhow!("KDF error: {}", e))?;

        let nonce_bytes = &data[0..12];
        let ciphertext = &data[12..];
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new_from_slice(&key)?;
        let plaintext = cipher
            .decrypt(nonce, Payload { msg: ciphertext, aad: b"" })
            .map_err(|e| anyhow::anyhow!("Decryption error: {}", e))?;

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption_roundtrip() {
        let data = b"halo dunia, ini data rahasia";
        let password = "password-super-kuat";

        let encrypted = EncryptionEngine::encrypt(data, password).expect("Enkripsi gagal");
        assert_ne!(data.to_vec(), encrypted);
        assert!(encrypted.len() > data.len());

        let decrypted = EncryptionEngine::decrypt(&encrypted, password).expect("Dekripsi gagal");
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_decryption_wrong_password() {
        let data = b"secret";
        let encrypted = EncryptionEngine::encrypt(data, "pass1").unwrap();
        let result = EncryptionEngine::decrypt(&encrypted, "wrong-pass");
        assert!(result.is_err());
    }
}
