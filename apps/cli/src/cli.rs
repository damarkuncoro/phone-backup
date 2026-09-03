pub use crate::subcommands::{Commands, ScheduleCommands};
use clap::Parser;

#[derive(Parser)]
#[command(name = "phone-backup", about = "Backup platform for Android devices")]
pub struct Cli {
    #[arg(short, long, default_value = "mock")]
    pub adapter: String,

    #[arg(long, default_value = "local")]
    pub storage: String,

    #[arg(long, env = "S3_BUCKET")]
    pub s3_bucket: Option<String>,

    #[arg(long, env = "S3_REGION")]
    pub s3_region: Option<String>,

    #[arg(long, env = "S3_ENDPOINT")]
    pub s3_endpoint: Option<String>,

    #[arg(long, env = "S3_ACCESS_KEY")]
    pub s3_access_key: Option<String>,

    #[arg(long, env = "S3_SECRET_KEY")]
    pub s3_secret_key: Option<String>,

    #[arg(long, env = "GCS_BUCKET")]
    pub gcs_bucket: Option<String>,

    #[arg(long, env = "GCS_CREDENTIAL")]
    pub gcs_credential: Option<String>,

    #[arg(long, env = "AZURE_CONTAINER")]
    pub azure_container: Option<String>,

    #[arg(long, env = "AZURE_ACCOUNT_NAME")]
    pub azure_account_name: Option<String>,

    #[arg(long, env = "AZURE_ACCOUNT_KEY")]
    pub azure_account_key: Option<String>,

    #[arg(long, env = "WEBDAV_ENDPOINT")]
    pub webdav_endpoint: Option<String>,

    #[arg(long, env = "WEBDAV_USER")]
    pub webdav_user: Option<String>,

    #[arg(long, env = "WEBDAV_PASSWORD")]
    pub webdav_password: Option<String>,

    /// Public key for asymmetric encryption
    #[arg(long, env = "PB_PUBKEY")]
    pub pubkey: Option<String>,

    /// Private key for asymmetric decryption
    #[arg(long, env = "PB_PRIVKEY")]
    pub privkey: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}
