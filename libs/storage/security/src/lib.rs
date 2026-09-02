pub mod config;
pub mod strategies;

use anyhow::Result;
use argon2::Argon2;
use hkdf::Hkdf;
use secrecy::ExposeSecret;
use sha2::Sha256;
use std::io::{Read, Write};

pub use config::EncryptionAlgorithm;
pub use strategies::EncryptionStrategy;

pub struct ExpertSecurity;

impl ExpertSecurity {
    pub fn get_strategy(algo: EncryptionAlgorithm) -> Box<dyn EncryptionStrategy> {
        match algo {
            EncryptionAlgorithm::None => Box::new(strategies::none::NoEncryptionStrategy),
            EncryptionAlgorithm::Aes256Gcm => Box::new(strategies::aes::AesGcmStrategy),
            EncryptionAlgorithm::XChaCha20Poly1305 => Box::new(strategies::chacha::ChaChaStrategy),
        }
    }

    /// Derives a chunk-specific encryption key using HKDF-SHA256 as per V4.0 spec.
    pub fn derive_chunk_key(master_key: &[u8], chunk_hash: &[u8]) -> Vec<u8> {
        let hk = Hkdf::<Sha256>::new(Some(master_key), chunk_hash);
        let mut okm = [0u8; 32];
        hk.expand(b"phone-backup-v4-chunk-key", &mut okm)
            .expect("HKDF expansion failed");
        okm.to_vec()
    }

    /// Derives a 256-bit database key from a password using Argon2id.
    pub fn derive_database_key(password: &str, salt: &[u8]) -> Result<String> {
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| anyhow::anyhow!("Argon2 error: {}", e))?;
        Ok(key.iter().map(|b| format!("{:02x}", b)).collect())
    }

    /// Expert-level symmetric encryption with raw key and algorithm choice.
    pub fn encrypt_raw(data: &[u8], key: &[u8], algo: EncryptionAlgorithm) -> Result<Vec<u8>> {
        Self::get_strategy(algo).encrypt(data, key)
    }

    /// Expert-level symmetric decryption with raw key and algorithm choice.
    pub fn decrypt_raw(data: &[u8], key: &[u8], algo: EncryptionAlgorithm) -> Result<Vec<u8>> {
        Self::get_strategy(algo).decrypt(data, key)
    }

    // --- LEGACY / CONVENIENCE WRAPPERS ---

    /// Convenience for password-based encryption (defaulting to AES-256-GCM for legacy compatibility).
    pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
        let salt = b"static_salt_for_demo";
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| anyhow::anyhow!("KDF error: {}", e))?;

        Self::encrypt_raw(data, &key, EncryptionAlgorithm::Aes256Gcm)
    }

    /// Convenience for password-based decryption.
    pub fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
        let salt = b"static_salt_for_demo";
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| anyhow::anyhow!("KDF error: {}", e))?;

        Self::decrypt_raw(data, &key, EncryptionAlgorithm::Aes256Gcm)
    }

    // --- ASYMMETRIC (X25519 via age) ---

    pub fn generate_keypair() -> (String, String) {
        let secret = age::x25519::Identity::generate();
        let public = secret.to_public();
        (
            secret.to_string().expose_secret().to_string(),
            public.to_string(),
        )
    }

    pub fn encrypt_with_key(data: &[u8], public_key: &str) -> Result<Vec<u8>> {
        let recipient: age::x25519::Recipient = public_key
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid public key format"))?;
        let mut encrypted = vec![];
        let encryptor = age::Encryptor::with_recipients(vec![Box::new(recipient)])
            .ok_or_else(|| anyhow::anyhow!("Failed to create encryptor"))?;
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(data)?;
        writer.finish()?;
        Ok(encrypted)
    }

    pub fn decrypt_with_key(data: &[u8], secret_key: &str) -> Result<Vec<u8>> {
        let identity: age::x25519::Identity = secret_key
            .parse()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hkdf_derivation_consistency() {
        let master = b"master-key";
        let chunk = b"chunk-hash";
        let key1 = ExpertSecurity::derive_chunk_key(master, chunk);
        let key2 = ExpertSecurity::derive_chunk_key(master, chunk);
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_chacha_roundtrip() {
        let data = b"secret data";
        let key = vec![0u8; 32];
        let enc = ExpertSecurity::encrypt_raw(data, &key, EncryptionAlgorithm::XChaCha20Poly1305)
            .unwrap();
        let dec = ExpertSecurity::decrypt_raw(&enc, &key, EncryptionAlgorithm::XChaCha20Poly1305)
            .unwrap();
        assert_eq!(data.to_vec(), dec);
    }
}
