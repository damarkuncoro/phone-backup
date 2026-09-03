use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VCardVersion {
    V2_1,
    #[default]
    V3_0,
    V4_0,
}

impl VCardVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::V2_1 => "2.1",
            Self::V3_0 => "3.0",
            Self::V4_0 => "4.0",
        }
    }
}
