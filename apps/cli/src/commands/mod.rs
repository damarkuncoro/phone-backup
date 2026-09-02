pub mod backup;
pub mod command_trait;
pub mod device;
pub mod doctor;
pub mod restore;
pub mod schedule;
pub mod stats;

#[allow(unused_imports)]
pub use command_trait::CliCommand;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use application::BackupService;
use domain::EncryptionMode;

pub fn execute_command<D, S, R, T, A, DP, P>(
    cli: Cli,
    service: BackupService<D, S, R, T, A, DP, P>,
) -> Result<()>
where
    D: ports::DevicePort,
    S: ports::ScannerPort,
    R: ports::RepositoryPort,
    T: ports::StoragePort,
    A: ports::AppProviderPort,
    DP: ports::DataProviderPort,
    P: ports::ProgressPort,
{
    // Determine encryption mode from global CLI flags
    let encryption = if let Some(pk) = cli.pubkey {
        EncryptionMode::PublicKey(pk)
    } else if let Some(sk) = cli.privkey {
        EncryptionMode::PublicKey(sk) // Use for decryption
    } else {
        EncryptionMode::None
    };

    match cli.command {
        Commands::Keygen => {
            let (secret, public) = application::EncryptionEngine::generate_keypair();
            println!("New Key Pair Generated!");
            println!("-----------------------");
            println!("Public Key (PB_PUBKEY):  {}", public);
            println!("Secret Key (PB_PRIVKEY): {}", secret);
            println!("\nKeep your Secret Key safe! You will need it to restore your backups.");
        }
        Commands::RecoveryKit { output } => {
            let (secret, public) = application::EncryptionEngine::generate_keypair();
            let html = format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Phone Backup - Zero-Knowledge Emergency Recovery Kit</title>
    <style>
        @page {{ size: A4; margin: 20mm; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; color: #0f172a; line-height: 1.5; padding: 20px; }}
        .badge {{ display: inline-block; padding: 4px 12px; background: #e0e7ff; color: #4338ca; border-radius: 9999px; font-weight: 800; font-size: 11px; text-transform: uppercase; letter-spacing: 0.05em; }}
        .header {{ border-bottom: 2px solid #e2e8f0; padding-bottom: 16px; margin-bottom: 24px; }}
        h1 {{ margin: 8px 0 4px 0; font-size: 24px; font-weight: 900; letter-spacing: -0.02em; }}
        p.subtitle {{ margin: 0; color: #64748b; font-size: 13px; }}
        .box {{ background: #f8fafc; border: 1.5px solid #cbd5e1; border-radius: 12px; padding: 16px; margin-bottom: 20px; }}
        .label {{ font-size: 11px; font-weight: 800; text-transform: uppercase; color: #64748b; margin-bottom: 4px; }}
        .key {{ font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; font-size: 13px; font-weight: 700; color: #0f172a; word-break: break-all; background: #ffffff; border: 1px solid #e2e8f0; padding: 8px 12px; border-radius: 6px; }}
        .warning {{ background: #fff1f2; border: 1.5px solid #fecdd3; border-radius: 12px; padding: 16px; margin-bottom: 24px; color: #9f1239; }}
        .warning h3 {{ margin: 0 0 4px 0; font-size: 14px; font-weight: 800; }}
        .warning p {{ margin: 0; font-size: 12px; }}
        .steps {{ margin-top: 24px; }}
        .step-item {{ margin-bottom: 12px; font-size: 12px; color: #334155; }}
        .code {{ font-family: monospace; background: #f1f5f9; padding: 2px 6px; border-radius: 4px; font-weight: 700; }}
        @media print {{
            body {{ padding: 0; }}
            .no-print {{ display: none; }}
        }}
    </style>
</head>
<body>
    <div class="header">
        <span class="badge">Cold Storage Recovery Document</span>
        <h1>Phone Backup Emergency Recovery Kit</h1>
        <p class="subtitle">Dokumen Resmi Pemulihan Enkripsi Asimetris X25519 (age) - Dicetak pada {date}</p>
    </div>

    <div class="warning">
        <h3>⚠️ PERINGATAN KEAMANAN TINGKAT TINGGI (SANGAT RAHASIA)</h3>
        <p>Kunci Rahasia (Secret Key) ini adalah satu-satunya cara untuk memulihkan dan mendekripsi seluruh data cadangan Anda jika komputer mengalami kerusakan atau kehilangan. Simpan lembaran ini di tempat fisik yang aman (brankas/cold storage).</p>
    </div>

    <div class="box">
        <div class="label">Public Key (Identitas Enkripsi Publik)</div>
        <div class="key">{public}</div>
    </div>

    <div class="box">
        <div class="label">Secret Key (Kunci Privat Dekripsi Darurat)</div>
        <div class="key">{secret}</div>
    </div>

    <div class="steps">
        <h3 style="font-size: 14px; font-weight: 800; margin-bottom: 10px;">Petunjuk Pemulihan Data Darurat:</h3>
        <div class="step-item"><strong>Langkah 1:</strong> Pasang binary CLI <span class="code">phone-backup</span> pada komputer baru.</div>
        <div class="step-item"><strong>Langkah 2:</strong> Ekspor kunci rahasia Anda pada terminal: <span class="code">export PB_PRIVKEY="{secret}"</span></div>
        <div class="step-item"><strong>Langkah 3:</strong> Jalankan perintah restorasi: <span class="code">phone-backup restore &lt;SNAPSHOT_ID&gt; --target /lokasi/pemulihan/</span></div>
    </div>
</body>
</html>"#, date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"), public = public, secret = secret);

            std::fs::write(&output, html)?;
            println!("✅ Emergency Recovery Kit berhasil diekspor ke: {}", output);
            println!("💡 Buka file tersebut di browser dan tekan Cmd+P / Ctrl+P untuk mencetak salinan fisik ke kertas (Cold Storage).");
        }
        Commands::Devices => device::print_devices(&service)?,
        Commands::DeviceInfo { id } => device::print_device_info(&service, &id)?,
        Commands::Scan { id } => device::scan_device(&service, &id)?,
        Commands::Apps { id } => device::list_apps(&service, &id)?,
        Commands::Backup {
            id,
            repo: _,
            password,
            include,
            exclude,
        } => {
            let enc = if let Some(pwd) = password {
                EncryptionMode::Password(pwd)
            } else {
                encryption
            };
            backup::run_backup(&service, &id, enc, include, exclude)?
        }
        Commands::Snapshots { id, snapshot } => {
            if let Some(s_id) = snapshot {
                backup::show_snapshot_detail(&service, &s_id)?;
            } else {
                backup::list_snapshots(&service, &id)?;
            }
        }
        Commands::Restore {
            snapshot_id,
            target,
            password,
            filter,
        } => {
            let enc = if let Some(pwd) = password {
                EncryptionMode::Password(pwd)
            } else {
                encryption
            };
            restore::run_restore(&service, &snapshot_id, target, enc, filter.as_deref())?
        }
        Commands::Verify { password } => {
            let enc = if let Some(pwd) = password {
                EncryptionMode::Password(pwd)
            } else {
                encryption
            };
            restore::run_verify(&service, enc)?
        }
        Commands::Stats => stats::run_stats(&service)?,
        Commands::Gc => {
            println!("🧹 Running Garbage Collection...");
            let deleted = service.garbage_collect()?;
            println!("✅ Done. Removed {} orphaned objects.", deleted);
        }
        Commands::Doctor => doctor::run_doctor(&service)?,
        Commands::Search { query } => stats::run_search(&service, &query)?,
        Commands::Contacts { query } => stats::run_contact_search(&service, &query)?,
        Commands::Sms { query } => stats::run_sms_search(&service, &query)?,
        Commands::Clone { source, target } => stats::run_clone(&service, &source, &target)?,
        Commands::Photos { id } => device::list_photos(&service, &id)?,
        Commands::Schedule { command } => {
            // Schedules might need encryption stored? For now pass None or default.
            schedule::handle_schedule(&service, command, EncryptionMode::None)?
        }
    }
    Ok(())
}
