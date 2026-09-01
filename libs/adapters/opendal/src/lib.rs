use anyhow::Result;
use opendal::{services, BlockingOperator, Operator};
use ports::StoragePort;
use std::io::Read;

pub struct CloudStorage {
    op: BlockingOperator,
}

impl CloudStorage {
    /// Create a new S3-based cloud storage adapter.
    pub fn new_s3(bucket: &str, region: &str, endpoint: &str, access_key: &str, secret_key: &str) -> Result<Self> {
        let builder = services::S3::default()
            .bucket(bucket)
            .region(region)
            .endpoint(endpoint)
            .access_key_id(access_key)
            .secret_access_key(secret_key);

        let op = Operator::new(builder)?.finish().blocking();
        Ok(Self { op })
    }

    /// Create a new GCS-based cloud storage adapter.
    pub fn new_gcs(bucket: &str, credential: &str) -> Result<Self> {
        let builder = services::Gcs::default()
            .bucket(bucket)
            .credential(credential);

        let op = Operator::new(builder)?.finish().blocking();
        Ok(Self { op })
    }

    /// Create a new Azure Blob-based cloud storage adapter.
    pub fn new_azblob(container: &str, endpoint: &str, account_name: &str, account_key: &str) -> Result<Self> {
        let builder = services::Azblob::default()
            .container(container)
            .endpoint(endpoint)
            .account_name(account_name)
            .account_key(account_key);

        let op = Operator::new(builder)?.finish().blocking();
        Ok(Self { op })
    }
}

impl StoragePort for CloudStorage {
    fn write(&self, id: &str, data: &mut dyn Read) -> Result<()> {
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer)?;
        self.op.write(id, buffer)?;
        Ok(())
    }

    fn read(&self, id: &str) -> Result<Box<dyn Read>> {
        let reader = self.op.read(id)?;
        Ok(Box::new(std::io::Cursor::new(reader.to_vec())))
    }

    fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.op.stat(id).is_ok())
    }

    fn delete(&self, id: &str) -> Result<()> {
        Ok(self.op.delete(id)?)
    }

    fn list(&self) -> Result<Vec<String>> {
        let lister = self.op.lister_with("/").recursive(true).call()?;
        let mut results = Vec::new();
        for entry in lister {
            let entry = entry?;
            if entry.metadata().is_file() {
                results.push(entry.path().to_string());
            }
        }
        Ok(results)
    }

    fn available_space(&self) -> Result<u64> {
        Ok(u64::MAX)
    }
}
