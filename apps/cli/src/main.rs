//! Composition root.
//!
//! This is the ONLY place in the workspace that is allowed to know
//! about a concrete adapter (`adapter_mock::MockDeviceAdapter` today,
//! `adapter_adb::AdbDeviceAdapter` from Phase 02 onward) and wire it
//! into `application::BackupService`. Everything above this file
//! programs against the `ports::DevicePort` trait only.

mod cli;
mod commands;

use adapter_adb::{AdbAppProvider, AdbDataProvider, AdbDeviceAdapter, AdbScannerAdapter};
use adapter_database_sqlite::SqliteRepository;
use adapter_filesystem::LocalStorage;
use adapter_mock::{MockAppProvider, MockDataProvider, MockDeviceAdapter, MockScannerAdapter};
use anyhow::Result;
use application::BackupService;
use clap::Parser;
use cli::Cli;
use commands::execute_command;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize repository (metadata)
    let repository = SqliteRepository::new("backup.db")?;

    // Initialize storage
    let storage_type = cli.storage.as_str();

    macro_rules! run_with_storage {
        ($storage:expr) => {
            match cli.adapter.as_str() {
                "adb" => {
                    let service = BackupService::new(
                        AdbDeviceAdapter::new(),
                        AdbScannerAdapter::new(),
                        repository,
                        $storage,
                        AdbAppProvider::new(),
                        AdbDataProvider::new(),
                    );
                    execute_command(cli.command, service)
                }
                _ => {
                    let service = BackupService::new(
                        MockDeviceAdapter::default(),
                        MockScannerAdapter::default(),
                        repository,
                        $storage,
                        MockAppProvider,
                        MockDataProvider,
                    );
                    execute_command(cli.command, service)
                }
            }
        };
    }

    match storage_type {
        "s3" => {
            use adapter_opendal::CloudStorage;
            let bucket = cli.s3_bucket.as_deref().unwrap_or("");
            let region = cli.s3_region.as_deref().unwrap_or("us-east-1");
            let endpoint = cli.s3_endpoint.as_deref().unwrap_or("");
            let access = cli.s3_access_key.as_deref().unwrap_or("");
            let secret = cli.s3_secret_key.as_deref().unwrap_or("");

            let storage = CloudStorage::new_s3(bucket, region, endpoint, access, secret)?;
            run_with_storage!(storage)
        }
        _ => {
            let storage = LocalStorage::new("backups")?;
            run_with_storage!(storage)
        }
    }
}
