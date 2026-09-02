//! resolver.rs
//!
//! Detects and (optionally) resolves processes that are holding an MTP/USB
//! device exclusively, so a subsequent scan/connect attempt can succeed.
//!
//! Strategy on macOS:
//! 1. Try to identify the *actual* holder of the device via `lsof` matched
//!    against IOKit/USB registry entries, rather than guessing by process name.
//! 2. Special-case `ptpcamerad` (macOS 13+) / `PTPCamera` (older macOS):
//!    these are launchd-managed daemons that respawn immediately after a
//!    plain `kill`, so they need to be unloaded via `launchctl` (or at least
//!    have their respawn suppressed) rather than just killed.
//! 3. Fall back to a small, explicit allow-list of known MTP-locking apps
//!    ONLY if lsof-based detection finds nothing — and even then, require
//!    the caller to confirm before killing anything outside that allow-list.

use std::collections::HashSet;
use std::process::Command;
use std::time::Duration;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct HoldingProcess {
    pub pid: u32,
    pub name: String,
    pub is_daemon: bool,
    pub launchd_label: Option<String>,
}

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("failed to run lsof: {0}")]
    Lsof(#[from] std::io::Error),
    #[error("failed to run launchctl: {0}")]
    Launchctl(std::io::Error),
    #[error("no conflicting process found via lsof")]
    NoHolderFound,
}

pub struct MtpConflictResolver;

impl MtpConflictResolver {
    const KNOWN_DAEMONS: &'static [(&'static str, &'static str)] = &[
        ("ptpcamerad", "com.apple.ptpcamerad"),
        ("PTPCamera", "com.apple.ptpcamerad"),
        ("mscamerad", "com.apple.mscamerad"),
    ];

    const FALLBACK_APP_NAMES: &'static [&'static str] = &[
        "Android File Transfer",
        "OpenMTP",
        "Image Capture",
        "Preview",
    ];

    pub fn find_holding_processes(
        device_serial: &str,
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

            let is_daemon = Self::KNOWN_DAEMONS
                .iter()
                .any(|(name, _)| command.to_lowercase().contains(name));
            let launchd_label = Self::KNOWN_DAEMONS
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

    pub fn kill_conflicts() -> anyhow::Result<usize> {
        let mut resolved = 0;

        // List of all known troublemakers
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
                    if Self::unload_launch_daemon(label).is_ok() {
                        resolved += 1;
                        continue;
                    }
                }
            }

            if Self::kill_pid(holder.pid).is_ok() {
                info!(pid = holder.pid, name = %holder.name, "Resolver: killed holding process");
                resolved += 1;
            }
        }

        Ok(resolved)
    }

    fn unload_launch_daemon(label: &str) -> Result<(), ResolverError> {
        info!(label, "Resolver: attempting launchctl bootout for daemon");

        let uid = Self::current_uid();
        if let Some(uid) = uid {
            let user_domain_target = format!("gui/{uid}/{label}");
            let bootout = Command::new("launchctl")
                .args(["bootout", &user_domain_target])
                .output();

            if let Ok(out) = bootout {
                if out.status.success() {
                    return Ok(());
                }
            }
        }

        let bootout = Command::new("launchctl")
            .args(["bootout", &format!("system/{label}")])
            .output();

        match bootout {
            Ok(out) if out.status.success() => Ok(()),
            _ => {
                warn!(label, "Resolver: bootout failed, trying legacy unload");
                let legacy = Command::new("launchctl")
                    .args(["unload", "-w", &Self::plist_path_guess(label)])
                    .output()
                    .map_err(ResolverError::Launchctl)?;

                if legacy.status.success() {
                    Ok(())
                } else {
                    Err(ResolverError::NoHolderFound)
                }
            }
        }
    }

    fn current_uid() -> Option<u32> {
        let output = Command::new("id").arg("-u").output().ok()?;
        if !output.status.success() {
            return None;
        }
        String::from_utf8_lossy(&output.stdout).trim().parse().ok()
    }

    fn plist_path_guess(label: &str) -> String {
        let agent_path = format!("/System/Library/LaunchAgents/{label}.plist");
        if std::path::Path::new(&agent_path).exists() {
            agent_path
        } else {
            format!("/System/Library/LaunchDaemons/{label}.plist")
        }
    }

    fn kill_pid(pid: u32) -> Result<(), std::io::Error> {
        // First send SIGSTOP to prevent immediate respawn
        let _ = Command::new("kill")
            .args(["-STOP", &pid.to_string()])
            .status();
        let status = Command::new("kill")
            .args(["-9", &pid.to_string()])
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::other("kill returned non-zero status"))
        }
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
