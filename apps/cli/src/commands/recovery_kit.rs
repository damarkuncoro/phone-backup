use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_recovery_kit<P: AsRef<Path>>(output_path: P) -> Result<()> {
    let output_path = output_path.as_ref();
    let (secret, public) = application::EncryptionEngine::generate_keypair();
    let now = chrono::Utc::now().to_rfc2822();

    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Phone Backup - Emergency Recovery Kit</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; margin: 40px; color: #1e293b; line-height: 1.5; }}
        .header {{ border-bottom: 2px solid #0284c7; padding-bottom: 12px; margin-bottom: 24px; }}
        h1 {{ color: #0f172a; margin: 0 0 6px 0; }}
        .date {{ color: #64748b; font-size: 14px; }}
        .alert {{ background: #fef2f2; border: 1px solid #fecaca; color: #991b1b; padding: 14px; border-radius: 8px; margin-bottom: 24px; font-weight: 500; }}
        .box {{ background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 16px; margin-bottom: 20px; }}
        .label {{ font-size: 13px; font-weight: 600; text-transform: uppercase; color: #475569; margin-bottom: 6px; }}
        .key {{ font-family: monospace; font-size: 14px; word-break: break-all; background: #fff; padding: 10px; border: 1px solid #cbd5e1; border-radius: 6px; }}
        .instructions {{ font-size: 14px; }}
        .cmd {{ background: #0f172a; color: #38bdf8; font-family: monospace; padding: 10px; border-radius: 6px; margin-top: 8px; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>📱 Phone Backup — Emergency Recovery Kit</h1>
        <div class="date">Generated on: {now}</div>
    </div>
    <div class="alert">
        ⚠️ CONFIDENTIAL DOCUMENT: Print this document and store it in a secure offline location (e.g., safe). Anyone with access to this secret key can decrypt your backups.
    </div>
    <div class="box">
        <div class="label">Public Encryption Key (PB_PUBKEY)</div>
        <div class="key">{public}</div>
    </div>
    <div class="box">
        <div class="label">Secret Decryption Key (PB_PRIVKEY)</div>
        <div class="key">{secret}</div>
    </div>
    <div class="box instructions">
        <div class="label">How to Restore Offline</div>
        <p>Run the following command to restore your encrypted snapshot:</p>
        <div class="cmd">phone-backup restore --private-key "{secret}" &lt;SNAPSHOT_ID&gt; -t ./restored_folder</div>
    </div>
</body>
</html>"#,
        now = now,
        public = public,
        secret = secret
    );

    let mut file = File::create(output_path)?;
    file.write_all(html_content.as_bytes())?;

    println!("✅ Emergency Recovery Kit generated at: {}", output_path.display());
    println!("🔑 Public Key:  {}", public);
    println!("🔐 Secret Key:  {}", secret);
    println!("💡 Please store or print this recovery document in a secure location.");

    Ok(())
}
