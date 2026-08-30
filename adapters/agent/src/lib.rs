pub mod adapter;
pub mod protocol;

pub use adapter::{AgentAdapter, AgentSessionManager};
pub use protocol::{AgentFileScanResponse, AgentHandshake, AgentHeartbeat, AgentStructuredDataResponse};
