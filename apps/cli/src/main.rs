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

use adapter_adb::{AdbAppProvider, AdbDataProvider, AdbDeviceAdapter, AdbScannerAdapter};
use adapter_database_sqlite::SqliteRepository;
use adapter_mock::{MockAppProvider, MockDataProvider, MockDeviceAdapter, MockScannerAdapter};
use anyhow::Result;
use application::BackupService;
use clap::Parser;
use cli::Cli;
use commands::execute_command;
use factory::StorageFactory;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize repository (metadata)
    let repository = SqliteRepository::new("backup.db")?;

    // Initialize storage using Factory Pattern
    let storage = StorageFactory::create_storage(&cli)?;

    match cli.adapter.as_str() {
        "adb" => {
            let service = BackupService::new(
                AdbDeviceAdapter::new(),
                AdbScannerAdapter::new(),
                repository,
                storage,
                AdbAppProvider::new(),
                AdbDataProvider::new(),
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
            );
            execute_command(cli, service)
        }
    }
}
