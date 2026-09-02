#[cfg(feature = "derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "derive", derive(Serialize, Deserialize))]
pub enum CompressionAlgorithm {
    None,
    #[default]
    Zstd,
}
