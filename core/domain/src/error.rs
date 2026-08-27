use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("device not found: {0}")]
    DeviceNotFound(String),

    #[error("capability not available: {0:?}")]
    CapabilityUnavailable(String),

    #[error("invalid state: {0}")]
    InvalidState(String),
}
