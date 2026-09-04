use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, Context, Result};
use flate2::read::ZlibDecoder;
use hkdf::Hkdf;
use sha2::Sha256;
use std::io::Read;

pub struct WhatsAppCryptDecryptor;

impl WhatsAppCryptDecryptor {
    /// Decrypts a WhatsApp crypt14/crypt15 encrypted database using a 64-digit hex key (32 bytes).
    pub fn decrypt_with_hex_key(encrypted_bytes: &[u8], hex_key: &str) -> Result<Vec<u8>> {
        let clean_hex = hex_key.replace([' ', '\n', '\r', '\t'], "");
        let raw_key = hex::decode(&clean_hex).context("Invalid 64-digit hex key format")?;
        if raw_key.len() != 32 {
            return Err(anyhow!("Key must be exactly 32 bytes (64 hex characters), got {} bytes", raw_key.len()));
        }

        Self::decrypt_with_key(encrypted_bytes, &raw_key)
    }

    /// Decrypts crypt14/crypt15 file bytes with a 32-byte key.
    pub fn decrypt_with_key(encrypted_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>> {
        if encrypted_bytes.len() < 256 {
            return Err(anyhow!("Encrypted database is too small ({} bytes)", encrypted_bytes.len()));
        }

        // Try derived keys (HKDF WhatsApp backup encryption) and raw key
        let mut key_candidates = Vec::new();
        key_candidates.push(key.to_vec());

        // Derived key 1: HKDF with WhatsApp Backup Encryption info
        let hk = Hkdf::<Sha256>::new(None, key);
        let mut derived1 = [0u8; 32];
        if hk.expand(b"WhatsApp Backup Encryption", &mut derived1).is_ok() {
            key_candidates.push(derived1.to_vec());
        }

        // Derived key 2: HKDF with backup info
        let mut derived2 = [0u8; 32];
        if hk.expand(b"backup encryption", &mut derived2).is_ok() {
            key_candidates.push(derived2.to_vec());
        }

        // Search for header offset candidates (standard crypt14 offsets: 191, 190, 128, 64, 51)
        let header_offsets = [191, 190, 192, 128, 67, 51, 143, 0];

        for k in &key_candidates {
            let cipher = match Aes256Gcm::new_from_slice(k) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for &offset in &header_offsets {
                if offset >= encrypted_bytes.len() {
                    continue;
                }

                // Potential IV locations: before ciphertext or inside header
                let iv_candidates = [
                    // IV immediately preceding ciphertext
                    if offset >= 16 { Some(&encrypted_bytes[offset - 16..offset]) } else { None },
                    // IV inside header at 67..83
                    if encrypted_bytes.len() >= 83 { Some(&encrypted_bytes[67..83]) } else { None },
                    // IV inside header at 143..159
                    if encrypted_bytes.len() >= 159 { Some(&encrypted_bytes[143..159]) } else { None },
                    // IV at start of file 0..16
                    Some(&encrypted_bytes[0..16]),
                    // IV at 51..67
                    if encrypted_bytes.len() >= 67 { Some(&encrypted_bytes[51..67]) } else { None },
                ];

                for iv_opt in iv_candidates.into_iter().flatten() {
                    if iv_opt.len() < 12 {
                        continue;
                    }
                    // Nonce is 12 bytes for standard AES-GCM (or 16 bytes truncated to 12)
                    let nonce_12 = Nonce::from_slice(&iv_opt[..12]);

                    let ciphertext = &encrypted_bytes[offset..];
                    if let Ok(decrypted) = cipher.decrypt(nonce_12, ciphertext) {
                        if let Ok(decompressed) = Self::try_decompress(&decrypted) {
                            if decompressed.starts_with(b"SQLite format 3\0") {
                                tracing::info!("Successfully decrypted WhatsApp SQLite DB ({} bytes)", decompressed.len());
                                return Ok(decompressed);
                            }
                        }
                        if decrypted.starts_with(b"SQLite format 3\0") {
                            return Ok(decrypted);
                        }
                    }
                }
            }
        }

        Err(anyhow!("Failed to decrypt database. Please verify the 64-digit key matches this WhatsApp account backup."))
    }

    fn try_decompress(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }
}
