use std::process::Command;
use tracing::{info, warn};

use super::types::ResolverError;

pub struct LaunchdManager;

impl LaunchdManager {
    pub fn unload_launch_daemon(label: &str) -> Result<(), ResolverError> {
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

    pub fn kill_pid(pid: u32) -> Result<(), std::io::Error> {
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
}
