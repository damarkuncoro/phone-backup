# phone-backup-security 🛡️

Expert security and encryption library for the phone-backup engine. This library implements the V4.0 security specifications, focusing on **Convergent Encryption** and multi-tier key derivation.

## Features

- **Expert Strategy Pattern**: Supports multiple encryption algorithms (`XChaCha20Poly1305`, `Aes256Gcm`, `None`).
- **Key Derivation (HKDF-SHA256)**: Implements chunk-specific key derivation to ensure high security while maintaining deduplication capabilities.
- **Argon2id Support**: Secure derivation of database encryption keys from user passwords.
- **Asymmetric Encryption**: Integration with `age` (X25519) for public-key based backup encryption.
- **Zeroize Integration**: Sensitive keys are cleared from memory when no longer needed.

## Architecture

This library is part of the `libs/storage` modular layer, providing cryptographic services to the application layer without exposing implementation details of the underlying providers.

## Usage

```rust
use security::{ExpertSecurity, EncryptionAlgorithm};

let data = b"sensitive information";
let key = vec![0u8; 32];

// Expert symmetric encryption
let encrypted = ExpertSecurity::encrypt_raw(data, &key, EncryptionAlgorithm::XChaCha20Poly1305)?;

// HKDF Key Derivation for V4.0 Chunks
let chunk_key = ExpertSecurity::derive_chunk_key(&master_key, &chunk_hash);
```
