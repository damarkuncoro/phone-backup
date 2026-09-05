use crate::domain::{SecurityType, WifiNetworkItem, WifiStats};
use std::collections::HashMap;

/// Domain service for Wi-Fi network analytics and filtering
pub struct WifiAnalytics;

impl WifiAnalytics {
    /// Compute summary statistics for a collection of Wi-Fi networks
    pub fn compute_stats(networks: &[WifiNetworkItem]) -> WifiStats {
        let mut stats = WifiStats {
            total_networks: networks.len(),
            ..Default::default()
        };

        let mut dist: HashMap<SecurityType, usize> = HashMap::new();

        for net in networks {
            *dist.entry(net.security_type).or_insert(0) += 1;

            if net.security_type.is_secure() {
                stats.secured_networks += 1;
            } else {
                stats.open_networks += 1;
            }

            if net.is_hidden {
                stats.hidden_networks += 1;
            }

            if net.is_metered {
                stats.metered_networks += 1;
            }

            if net.auto_connect {
                stats.auto_connect_count += 1;
            }
        }

        stats.security_distribution = dist;
        stats
    }

    /// Filter networks by security type, query string, or hidden status
    pub fn filter_networks(
        networks: Vec<WifiNetworkItem>,
        security: Option<SecurityType>,
        query: Option<&str>,
        hidden_only: bool,
    ) -> Vec<WifiNetworkItem> {
        networks
            .into_iter()
            .filter(|n| {
                if let Some(sec) = security {
                    if n.security_type != sec {
                        return false;
                    }
                }
                if hidden_only && !n.is_hidden {
                    return false;
                }
                if let Some(q) = query {
                    let q_lower = q.to_lowercase();
                    if !n.ssid.to_lowercase().contains(&q_lower) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
