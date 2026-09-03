//! Composition root.
//!
//! This is the ONLY place in the workspace that is allowed to know
//! about a concrete adapter (`adapter_mock::MockDeviceAdapter` today,
//! `adapter_adb::AdbDeviceAdapter` from Phase 02 onward) and wire it
//! into `application::BackupService`. Everything above this file
//! programs against the `ports::DevicePort` trait only.

mod cli;
mod commands;
mod factory;
mod progress;
mod subcommands;

use adapter_adb::{AdbAdapter, AdbClient};
use adapter_database_sqlite::SqliteRepository;
use adapter_mock::{MockAppProvider, MockDataProvider, MockDeviceAdapter, MockScannerAdapter};
use anyhow::Result;
use application::BackupService;
use clap::Parser;
use cli::Cli;
use commands::execute_command;
use factory::StorageFactory;
use progress::CliProgress;

#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
fn reset_sigpipe() {}

fn main() -> Result<()> {
    reset_sigpipe();

    // Initialize structured logging to terminal and file
    let file_appender = tracing_appender::rolling::daily("workspace/logs", "phone-backup.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into())
            // Silence noisy third-party libraries
            .add_directive("nusb=off".parse().unwrap())
            .add_directive("mtp_rs=warn".parse().unwrap()),
        )
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(fmt::layer().with_ansi(false).with_writer(non_blocking))
        .init();

    let cli = Cli::parse();

    // Initialize repository (metadata)
    let repository = SqliteRepository::new("workspace/backup.db")?;

    // Initialize storage using Factory Pattern
    let storage = StorageFactory::create_storage(&cli)?;

    match cli.adapter.as_str() {
        "adb" => {
            let adb_client = AdbClient::new();
            let adb_adapter = AdbAdapter::new(adb_client);
            let service = BackupService::builder()
                .with_device_adapter(adb_adapter.clone())
                .with_scanner_adapter(adb_adapter.clone())
                .with_repository(repository)
                .with_storage(storage)
                .with_app_provider(adb_adapter.clone())
                .with_data_provider(adb_adapter)
                .with_progress(CliProgress::new())
                .build()?;
            execute_command(cli, service)
        }
        "agent" => {
            let agent_adapter = adapter_agent::AgentAdapter::default();
            let service = BackupService::builder()
                .with_device_adapter(agent_adapter.clone())
                .with_scanner_adapter(agent_adapter.clone())
                .with_repository(repository)
                .with_storage(storage)
                .with_app_provider(agent_adapter.clone())
                .with_data_provider(agent_adapter)
                .with_progress(CliProgress::new())
                .build()?;
            execute_command(cli, service)
        }
        "mtp" => {
            let mtp_adapter = adapter_mtp::MtpAdapter::default();
            let service = BackupService::builder()
                .with_device_adapter(mtp_adapter.clone())
                .with_scanner_adapter(mtp_adapter.clone())
                .with_repository(repository)
                .with_storage(storage)
                .with_app_provider(adapter_mock::MockAppProvider)
                .with_data_provider(adapter_mock::MockDataProvider)
                .with_progress(CliProgress::new())
                .build()?;
            execute_command(cli, service)
        }
        "folder" => {
            // Treat current directory as the device root for testing
            let folder_adapter = adapter_mtp::MtpAdapter::with_root(std::env::current_dir()?);
            let service = BackupService::builder()
                .with_device_adapter(folder_adapter.clone())
                .with_scanner_adapter(folder_adapter.clone())
                .with_repository(repository)
                .with_storage(storage)
                .with_app_provider(adapter_mock::MockAppProvider)
                .with_data_provider(adapter_mock::MockDataProvider)
                .with_progress(CliProgress::new())
                .build()?;
            execute_command(cli, service)
        }
        _ => {
            let service = BackupService::builder()
                .with_device_adapter(MockDeviceAdapter::default())
                .with_scanner_adapter(MockScannerAdapter)
                .with_repository(repository)
                .with_storage(storage)
                .with_app_provider(MockAppProvider)
                .with_data_provider(MockDataProvider)
                .with_progress(CliProgress::new())
                .build()?;
            execute_command(cli, service)
        }
    }
}
