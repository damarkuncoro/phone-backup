use apps::{AppAuditFactory, AppManifestBuilder, AuditReportFormat};
use std::fs;

#[tauri::command(rename_all = "snake_case")]
pub async fn audit_apk_file(path: String) -> Result<String, String> {
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let manifest = if let Ok(strings) = apps::AxmlParser::extract_string_pool(&bytes) {
        let pkg_name = strings.iter().find(|s| s.contains('.')).cloned().unwrap_or_else(|| "com.unknown.app".to_string());
        let mut mb = AppManifestBuilder::new(pkg_name, 26, 34);
        for s in &strings {
            if s.starts_with("android.permission.") {
                mb = mb.add_permission(s.clone());
            }
        }
        mb.build()
    } else {
        AppManifestBuilder::new("com.app.audited", 28, 34)
            .add_permission("android.permission.INTERNET")
            .build()
    };

    AppAuditFactory::generate_report(&manifest, AuditReportFormat::Json).map_err(|e| e.to_string())
}
