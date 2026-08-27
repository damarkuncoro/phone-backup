use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A discrete thing the backup engine might want to read from a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Capability {
    ReadFiles,
    ReadMedia,
    ReadDownload,
    ReadDocuments,
    ReadAppData,
    ReadContacts,
    ReadSms,
    ReadCallLog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityStatus {
    Available,
    Denied,
    Unsupported,
    RequiresUserAction,
}

/// What a specific connected device can actually do.
///
/// Never assume all Android devices support the same set of
/// capabilities — this matrix is populated per-device by the
/// adapter layer during the discovery/permission phase.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapabilityMatrix {
    entries: BTreeMap<Capability, CapabilityStatus>,
}

impl CapabilityMatrix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, capability: Capability, status: CapabilityStatus) {
        self.entries.insert(capability, status);
    }

    pub fn status(&self, capability: Capability) -> CapabilityStatus {
        self.entries
            .get(&capability)
            .copied()
            .unwrap_or(CapabilityStatus::Unsupported)
    }

    pub fn is_available(&self, capability: Capability) -> bool {
        matches!(self.status(capability), CapabilityStatus::Available)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Capability, &CapabilityStatus)> {
        self.entries.iter()
    }
}
