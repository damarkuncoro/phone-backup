pub mod launchd;
pub mod lsof;
pub mod types;

use std::process::Command;
use std::time::Duration;
use tracing::info;

pub use launchd::LaunchdManager;
pub use lsof::LsofScanner;
pub use types::{HoldingProcess, ResolverError};

pub struct MtpConflictResolver;

impl MtpConflictResolver {
    pub const KNOWN_DAEMONS: &'static [(&'static str, &'static str)] = &[
        ("ptpcamerad", "com.apple.ptpcamerad"),
        ("PTPCamera", "com.apple.ptpcamerad"),
        ("mscamerad", "com.apple.mscamerad"),
    ];

    pub const FALLBACK_APP_NAMES: &'static [&'static str] = &[
        "Android File Transfer",
        "OpenMTP",
        "Image Capture",
        "Preview",
    ];

    pub fn find_holding_processes(
        device_serial: &str,
    ) -> Result<Vec<HoldingProcess>, ResolverError> {
        LsofScanner::find_holding_processes(device_serial, Self::KNOWN_DAEMONS)
    }

    pub fn kill_conflicts() -> anyhow::Result<usize> {
        let mut resolved = 0;
        let mut targets: Vec<String> = Self::KNOWN_DAEMONS
            .iter()
            .map(|d| d.0.to_string())
            .collect();
        targets.extend(Self::FALLBACK_APP_NAMES.iter().map(|s| s.to_string()));

        for name in targets {
            let status = Command::new("pkill")
                .args(["-9", "-i", "-f", &name])
                .status();

            if let Ok(s) = status {
                if s.success() {
                    info!("MTP Resolver: Aggressively killed '{}'", name);
                    resolved += 1;
                }
            }
        }
        Ok(resolved)
    }

    pub fn resolve_conflicts(device_serial: &str) -> Result<usize, ResolverError> {
        let holders = match Self::find_holding_processes(device_serial) {
            Ok(h) => h,
            Err(ResolverError::NoHolderFound) => Self::fallback_find_known_apps(),
            Err(e) => return Err(e),
        };

        let mut resolved = 0;
        for holder in holders {
            if holder.is_daemon {
                if let Some(label) = &holder.launchd_label {
                    if LaunchdManager::unload_launch_daemon(label).is_ok() {
                        resolved += 1;
                        continue;
                    }
                }
            }

            if LaunchdManager::kill_pid(holder.pid).is_ok() {
                info!(pid = holder.pid, name = %holder.name, "Resolver: killed holding process");
                resolved += 1;
            }
        }
        Ok(resolved)
    }

    pub fn find_conflicts() -> Vec<String> {
        Self::fallback_find_known_apps()
            .into_iter()
            .map(|h| h.name)
            .collect()
    }

    fn fallback_find_known_apps() -> Vec<HoldingProcess> {
        let output = Command::new("ps").args(["ax", "-o", "pid,comm"]).output();
        let text = match output {
            Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            Err(_) => return Vec::new(),
        };

        let mut result = Vec::new();
        for line in text.lines().skip(1) {
            let line = line.trim();
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() < 2 {
                continue;
            }

            let pid_str = fields[0];
            let name = fields[1..].join(" ");

            let is_daemon = Self::KNOWN_DAEMONS
                .iter()
                .any(|(dname, _)| name.to_lowercase().contains(&dname.to_lowercase()));
            let launchd_label = Self::KNOWN_DAEMONS
                .iter()
                .find(|(dname, _)| name.to_lowercase().contains(&dname.to_lowercase()))
                .map(|(_, label)| label.to_string());
            let is_known_app = Self::FALLBACK_APP_NAMES
                .iter()
                .any(|known| name.to_lowercase().contains(&known.to_lowercase()));

            if is_daemon || is_known_app {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    result.push(HoldingProcess {
                        pid,
                        name: name.to_string(),
                        is_daemon,
                        launchd_label,
                    });
                }
            }
        }
        result
    }

    pub async fn resolve_and_settle(device_serial: &str) -> Result<usize, ResolverError> {
        let n = Self::resolve_conflicts(device_serial)?;
        tokio::time::sleep(Duration::from_millis(400)).await;
        Ok(n)
    }
}
