mod data;
mod device;
mod scanner;

use crate::protocol::{AgentHandshake, AgentStructuredDataResponse};
use domain::FileEntry;
use std::sync::{Arc, RwLock};

/// In-memory state and registry of active Android Companion Agent sessions.
#[derive(Clone, Default)]
pub struct AgentSessionManager {
    pub(crate) devices: Arc<RwLock<Vec<AgentHandshake>>>,
    pub(crate) scanned_files: Arc<RwLock<Vec<FileEntry>>>,
    pub(crate) structured_data: Arc<RwLock<AgentStructuredDataResponse>>,
}

impl AgentSessionManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a newly connected Android Companion Agent.
    pub fn register_device(&self, handshake: AgentHandshake) {
        let mut devs = self.devices.write().unwrap();
        devs.retain(|d| d.device_id != handshake.device_id);
        devs.push(handshake);
    }

    /// Update cached file manifest for the connected agent.
    pub fn update_files(&self, files: Vec<FileEntry>) {
        let mut f = self.scanned_files.write().unwrap();
        *f = files;
    }

    /// Update structured data (Contacts, SMS, Call Logs) from the agent.
    pub fn update_structured_data(&self, data: AgentStructuredDataResponse) {
        let mut s = self.structured_data.write().unwrap();
        *s = data;
    }
}

/// Agent adapter implementing Hexagonal Ports for wireless Android Agent communication.
#[derive(Clone)]
pub struct AgentAdapter {
    pub(crate) session: AgentSessionManager,
}

impl AgentAdapter {
    pub fn new(session: AgentSessionManager) -> Self {
        Self { session }
    }

    pub fn with_default_session() -> Self {
        let session = AgentSessionManager::new();
        // Seed default wireless mock device for testing / standalone CLI mode
        session.register_device(AgentHandshake {
            device_id: "AGENT_WIRELESS_01".to_string(),
            manufacturer: "Xiaomi".to_string(),
            model: "22101316G (Wireless Agent)".to_string(),
            android_version: "Android 14 (HyperOS)".to_string(),
            storage_used_bytes: 41_481_015_296,
            storage_total_bytes: 242_017_599_488,
            battery_percent: Some(95),
            temperature_c: Some(34.5),
        });
        Self { session }
    }
}

impl Default for AgentAdapter {
    fn default() -> Self {
        Self::with_default_session()
    }
}
