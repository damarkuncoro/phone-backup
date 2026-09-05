use super::SecurityType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Summary statistics of backed up Wi-Fi networks
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WifiStats {
    pub total_networks: usize,
    pub secured_networks: usize,
    pub open_networks: usize,
    pub hidden_networks: usize,
    pub metered_networks: usize,
    pub auto_connect_count: usize,
    pub security_distribution: HashMap<SecurityType, usize>,
}
