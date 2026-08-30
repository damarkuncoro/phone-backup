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

fn main() -> Result<()> {
    // Initialize structured logging to terminal and file
    let file_appender = tracing_appender::rolling::daily("workspace/logs", "phone-backup.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
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
            let service = BackupService::new(
                adb_adapter.clone(),
                adb_adapter.clone(),
                repository,
                storage,
                adb_adapter.clone(),
                adb_adapter,
                CliProgress::new(),
            );
            execute_command(cli, service)
        }
        "agent" => {
            let agent_adapter = adapter_agent::AgentAdapter::default();
            let service = BackupService::new(
                agent_adapter.clone(),
                agent_adapter.clone(),
                repository,
                storage,
                agent_adapter.clone(),
                agent_adapter,
                CliProgress::new(),
            );
            execute_command(cli, service)
        }
        _ => {
            let service = BackupService::new(
                MockDeviceAdapter::default(),
                MockScannerAdapter::default(),
                repository,
                storage,
                MockAppProvider,
                MockDataProvider,
                CliProgress::new(),
            );
            execute_command(cli, service)
        }
    }
}
