use std::collections::HashSet;
use std::process::Command;
use tracing::info;

use super::types::{HoldingProcess, ResolverError};

pub struct LsofScanner;

impl LsofScanner {
    pub fn find_holding_processes(
        device_serial: &str,
        known_daemons: &[(&str, &str)],
    ) -> Result<Vec<HoldingProcess>, ResolverError> {
        info!(
            device_serial,
            "Resolver: searching for processes holding device via lsof"
        );

        let output = Command::new("lsof").args(["-n", "-P"]).output()?;
        let text = String::from_utf8_lossy(&output.stdout);

        let mut found = Vec::new();
        let mut seen_pids = HashSet::new();

        for line in text.lines() {
            if !line.to_lowercase().contains(&device_serial.to_lowercase())
                && !Self::line_matches_ptp_class(line)
            {
                continue;
            }

            let mut fields = line.split_whitespace();
            let command = fields.next().unwrap_or_default().to_string();
            let pid: u32 = match fields.next().and_then(|p| p.parse().ok()) {
                Some(p) => p,
                None => continue,
            };

            if !seen_pids.insert(pid) {
                continue;
            }

            let is_daemon = known_daemons
                .iter()
                .any(|(name, _)| command.to_lowercase().contains(name));
            let launchd_label = known_daemons
                .iter()
                .find(|(name, _)| command.to_lowercase().contains(name))
                .map(|(_, label)| label.to_string());

            found.push(HoldingProcess {
                pid,
                name: command,
                is_daemon,
                launchd_label,
            });
        }

        if found.is_empty() {
            return Err(ResolverError::NoHolderFound);
        }

        Ok(found)
    }

    fn line_matches_ptp_class(line: &str) -> bool {
        let l = line.to_lowercase();
        l.contains("ptp")
            || l.contains("mtp")
            || l.contains("appleusbcamera")
            || l.contains("imagecapturecore")
            || l.contains("mscamera")
    }
}
