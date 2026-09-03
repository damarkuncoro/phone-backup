use crate::model::{AppManifest, PermissionProtection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Safe => "Safe",
            Self::Low => "Low Risk",
            Self::Medium => "Medium Risk",
            Self::High => "High Risk",
            Self::Critical => "Critical / Privileged Risk",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub score: u32,
    pub level: RiskLevel,
    pub dangerous_permissions: Vec<String>,
    pub privacy_concerns: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct AppRiskAuditor;

impl AppRiskAuditor {
    pub fn audit(manifest: &AppManifest) -> RiskAssessment {
        let mut score: u32 = 0;
        let mut dangerous_permissions = Vec::new();
        let mut privacy_concerns = Vec::new();
        let mut warnings = Vec::new();

        if manifest.is_debuggable {
            score += 25;
            warnings.push("Application is debuggable in production (security vulnerability)".to_string());
        }

        if manifest.sdk.target_sdk < 29 {
            score += 15;
            warnings.push(format!("Application targets legacy Android SDK {}", manifest.sdk.target_sdk));
        }

        for perm in &manifest.permissions {
            match perm.protection_level {
                PermissionProtection::Dangerous => {
                    score += 10;
                    dangerous_permissions.push(perm.name.clone());
                }
                PermissionProtection::Privileged => {
                    score += 20;
                    warnings.push(format!("Requests privileged system permission: {}", perm.name));
                }
                _ => {}
            }

            if perm.is_critical_privacy {
                privacy_concerns.push(perm.name.clone());
            }
        }

        let level = match score {
            0..=10 => RiskLevel::Safe,
            11..=30 => RiskLevel::Low,
            31..=60 => RiskLevel::Medium,
            61..=90 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };

        RiskAssessment {
            score,
            level,
            dangerous_permissions,
            privacy_concerns,
            warnings,
        }
    }
}
