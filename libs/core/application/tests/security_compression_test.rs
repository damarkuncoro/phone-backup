use phone_backup_application::storage::compression::CompressionEngine;
use phone_backup_application::storage::security::EncryptionEngine;

#[test]
fn test_derive_database_key() {
    let pass = "my_master_db_password";
    let salt = b"phone_backup_db_salt";

    let key1 = EncryptionEngine::derive_database_key(pass, salt).unwrap();
    let key2 = EncryptionEngine::derive_database_key(pass, salt).unwrap();

    assert_eq!(key1.len(), 64);
    assert_eq!(key1, key2);

    let key_diff_pass = EncryptionEngine::derive_database_key("different_pass", salt).unwrap();
    assert_ne!(key1, key_diff_pass);
}

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

#[test]
fn test_asymmetric_roundtrip() {
    let data = b"ultra secret message with public key";
    let (secret, public) = EncryptionEngine::generate_keypair();

    let encrypted =
        EncryptionEngine::encrypt_with_key(data, &public).expect("Asymmetric encryption failed");
    assert_ne!(data.to_vec(), encrypted);

    let decrypted = EncryptionEngine::decrypt_with_key(&encrypted, &secret)
        .expect("Asymmetric decryption failed");
    assert_eq!(data.to_vec(), decrypted);
}

#[test]
fn test_asymmetric_wrong_key() {
    let data = b"secret";
    let (_, public) = EncryptionEngine::generate_keypair();
    let (wrong_secret, _) = EncryptionEngine::generate_keypair();

    let encrypted = EncryptionEngine::encrypt_with_key(data, &public).unwrap();
    let result = EncryptionEngine::decrypt_with_key(&encrypted, &wrong_secret);
    assert!(result.is_err());
}

#[test]
fn test_compression_decompression_roundtrip() {
    let data = b"data yang berulang-ulang ulang-ulang ulang-ulang";
    let compressed = CompressionEngine::compress(data).unwrap();
    assert!(compressed.len() > 0);

    let decompressed = CompressionEngine::decompress(&compressed).unwrap();
    assert_eq!(data.to_vec(), decompressed);
}

#[test]
fn test_should_compress_policy() {
    assert!(CompressionEngine::should_compress("text/plain"));
    assert!(CompressionEngine::should_compress("application/json"));
    assert!(!CompressionEngine::should_compress("image/jpeg"));
    assert!(!CompressionEngine::should_compress("video/mp4"));
}
