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
            "gcs" => {
                use adapter_opendal::CloudStorage;
                let bucket = cli.gcs_bucket.as_deref().unwrap_or("");
                let credential = cli.gcs_credential.as_deref().unwrap_or("");
                let storage = CloudStorage::new_gcs(bucket, credential)?;
                Ok(Box::new(storage))
            }
            "azure" => {
                use adapter_opendal::CloudStorage;
                let container = cli.azure_container.as_deref().unwrap_or("");
                let endpoint = cli.s3_endpoint.as_deref().unwrap_or(""); // Use S3 endpoint arg if shared or add specific
                let account = cli.azure_account_name.as_deref().unwrap_or("");
                let key = cli.azure_account_key.as_deref().unwrap_or("");
                let storage = CloudStorage::new_azblob(container, endpoint, account, key)?;
                Ok(Box::new(storage))
            }
            "webdav" | "nextcloud" => {
                use adapter_opendal::CloudStorage;
                let endpoint = cli.webdav_endpoint.as_deref().unwrap_or("");
                let user = cli.webdav_user.as_deref().unwrap_or("");
                let pass = cli.webdav_password.as_deref().unwrap_or("");
                let storage = CloudStorage::new_webdav(endpoint, user, pass)?;
                Ok(Box::new(storage))
            }
            _ => {
                let storage = LocalStorage::new("workspace/backups")?;
                Ok(Box::new(storage))
            }
        }
    }
}
