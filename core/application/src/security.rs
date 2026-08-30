use anyhow::Result;
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use argon2::Argon2;
use rand::{RngCore, thread_rng};

use secrecy::ExposeSecret;
use std::io::{Read, Write};

pub struct EncryptionEngine;

impl EncryptionEngine {
    /// Generate a new X25519 key pair for asymmetric encryption.
    /// Returns (Secret Key, Public Key) as strings.
    pub fn generate_keypair() -> (String, String) {
        let secret = age::x25519::Identity::generate();
        let public = secret.to_public();
        // age-x25519 uses secrecy, to_string() returns SecretString
        (secret.to_string().expose_secret().to_string(), public.to_string())
    }

    /// Encrypt data using a public key (X25519).
    pub fn encrypt_with_key(data: &[u8], public_key: &str) -> Result<Vec<u8>> {
        let recipient: age::x25519::Recipient = public_key.parse()
            .map_err(|_| anyhow::anyhow!("Invalid public key format"))?;

        let mut encrypted = vec![];
        let encryptor = age::Encryptor::with_recipients(vec![Box::new(recipient)])
            .ok_or_else(|| anyhow::anyhow!("Failed to create encryptor"))?;
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(data)?;
        writer.finish()?;

        Ok(encrypted)
    }

    /// Decrypt data using a secret key (X25519).
    pub fn decrypt_with_key(data: &[u8], secret_key: &str) -> Result<Vec<u8>> {
        let identity: age::x25519::Identity = secret_key.parse()
            .map_err(|_| anyhow::anyhow!("Invalid secret key format"))?;

        let decryptor = match age::Decryptor::new(data)? {
            age::Decryptor::Recipients(d) => d,
            _ => anyhow::bail!("Data is not encrypted with a key"),
        };

        let mut decrypted = vec![];
        let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
        reader.read_to_end(&mut decrypted)?;

        Ok(decrypted)
    }

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

    /// Derive a 256-bit hex-encoded database encryption key from a user passphrase using Argon2id.
    pub fn derive_database_key(passphrase: &str, salt: &[u8]) -> Result<String> {
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(passphrase.as_bytes(), salt, &mut key)
            .map_err(|e| anyhow::anyhow!("Argon2 KDF error: {}", e))?;

        let hex_key: String = key.iter().map(|b| format!("{:02x}", b)).collect();
        Ok(hex_key)
    }
}
