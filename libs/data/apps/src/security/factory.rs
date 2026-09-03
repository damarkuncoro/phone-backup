use super::risk_scorer::{AppRiskAuditor, RiskAssessment};
use crate::model::AppManifest;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditReportFormat {
    Json,
    Markdown,
    PlainText,
}

pub struct AppAuditFactory;

impl AppAuditFactory {
    pub fn generate_report(manifest: &AppManifest, format: AuditReportFormat) -> Result<String> {
        let assessment = AppRiskAuditor::audit(manifest);
        match format {
            AuditReportFormat::Json => Ok(serde_json::to_string_pretty(&assessment)?),
            AuditReportFormat::Markdown => Ok(Self::format_markdown(manifest, &assessment)),
            AuditReportFormat::PlainText => Ok(Self::format_plain(manifest, &assessment)),
        }
    }

    fn format_markdown(manifest: &AppManifest, assessment: &RiskAssessment) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Security Audit: {}\n\n", manifest.package_name));
        md.push_str(&format!("- **Version**: {} (Code: {})\n", manifest.version_name, manifest.version_code));
        md.push_str(&format!("- **Risk Level**: {} (Score: {})\n", assessment.level.label(), assessment.score));
        md.push_str(&format!("- **Target SDK**: {}\n\n", manifest.sdk.target_sdk));

        if !assessment.dangerous_permissions.is_empty() {
            md.push_str("### ⚠️ Dangerous Permissions:\n");
            for p in &assessment.dangerous_permissions {
                md.push_str(&format!("- `{}`\n", p));
            }
            md.push('\n');
        }

        if !assessment.warnings.is_empty() {
            md.push_str("### 🚨 Security Warnings:\n");
            for w in &assessment.warnings {
                md.push_str(&format!("- {}\n", w));
            }
        }

        md
    }

    fn format_plain(manifest: &AppManifest, assessment: &RiskAssessment) -> String {
        format!(
            "App: {} | Version: {} | Risk: {} ({}) | Dangerous Perms: {}",
            manifest.package_name,
            manifest.version_name,
            assessment.level.label(),
            assessment.score,
            assessment.dangerous_permissions.len()
        )
    }
}
