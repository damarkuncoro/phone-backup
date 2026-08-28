use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("device not found: {0}")]
    DeviceNotFound(String),

    #[error("capability not available: {0:?}")]
    CapabilityUnavailable(String),

    #[error("invalid state: {0}")]
    InvalidState(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("database error: {0}")]
    Database(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("encryption error: {0}")]
    Encryption(String),

    #[error("unauthorized access")]
    Unauthorized,
}
