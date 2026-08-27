use crate::cli::Cli;
use adapter_filesystem::LocalStorage;
use anyhow::Result;
use ports::StoragePort;

pub struct StorageFactory;

impl StorageFactory {
    pub fn create_storage(cli: &Cli) -> Result<Box<dyn StoragePort>> {
        match cli.storage.as_str() {
            "s3" => {
                use adapter_opendal::CloudStorage;
                let bucket = cli.s3_bucket.as_deref().unwrap_or("");
                let region = cli.s3_region.as_deref().unwrap_or("us-east-1");
                let endpoint = cli.s3_endpoint.as_deref().unwrap_or("");
                let access = cli.s3_access_key.as_deref().unwrap_or("");
                let secret = cli.s3_secret_key.as_deref().unwrap_or("");

                let storage = CloudStorage::new_s3(bucket, region, endpoint, access, secret)?;
                Ok(Box::new(storage))
            }
            _ => {
                let storage = LocalStorage::new("backups")?;
                Ok(Box::new(storage))
            }
        }
    }
}
