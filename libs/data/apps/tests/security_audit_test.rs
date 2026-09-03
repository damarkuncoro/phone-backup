use phone_backup_apps::{
    AppAuditFactory, AppManifestBuilder, AppRiskAuditor, AuditReportFormat, RiskLevel,
};

#[test]
fn test_security_audit_dangerous_permissions() {
    let manifest = AppManifestBuilder::new("com.suspicious.app", 26, 33)
        .add_permission("android.permission.READ_CONTACTS")
        .add_permission("android.permission.SEND_SMS")
        .add_permission("android.permission.ACCESS_FINE_LOCATION")
        .add_permission("android.permission.CAMERA")
        .add_permission("android.permission.INTERNET")
        .with_debuggable(true)
        .build();

    let assessment = AppRiskAuditor::audit(&manifest);

    assert!(assessment.score >= 50);
    assert_eq!(assessment.dangerous_permissions.len(), 4);
    assert!(assessment.warnings.iter().any(|w| w.contains("debuggable")));
    assert!(assessment.level >= RiskLevel::Medium);
}

#[test]
fn test_security_audit_safe_application() {
    let manifest = AppManifestBuilder::new("com.safe.calculator", 30, 34)
        .add_permission("android.permission.VIBRATE")
        .with_debuggable(false)
        .build();

    let assessment = AppRiskAuditor::audit(&manifest);

    assert_eq!(assessment.score, 0);
    assert_eq!(assessment.level, RiskLevel::Safe);
    assert!(assessment.dangerous_permissions.is_empty());
}

#[test]
fn test_audit_report_factory_markdown_and_json() {
    let manifest = AppManifestBuilder::new("com.bank.secure", 31, 34)
        .add_permission("android.permission.CAMERA")
        .add_permission("android.permission.READ_CONTACTS")
        .build();

    let md_report = AppAuditFactory::generate_report(&manifest, AuditReportFormat::Markdown)
        .expect("Markdown report generation failed");
    assert!(md_report.contains("# Security Audit: com.bank.secure"));
    assert!(md_report.contains("android.permission.CAMERA"));

    let json_report = AppAuditFactory::generate_report(&manifest, AuditReportFormat::Json)
        .expect("JSON report generation failed");
    assert!(json_report.contains("\"level\": \"Low\""));
}
