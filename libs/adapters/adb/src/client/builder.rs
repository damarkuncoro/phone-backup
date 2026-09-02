use crate::client::AdbClient;
use anyhow::Result;
use std::time::Duration;

/// Builder for AdbClient configuration
pub struct AdbClientBuilder {
    pub(crate) adb_path: Option<String>,
    pub(crate) timeout: Duration,
}

impl Default for AdbClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AdbClientBuilder {
    pub fn new() -> Self {
        Self {
            adb_path: None,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_adb_path(mut self, path: String) -> Self {
        self.adb_path = Some(path);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Result<AdbClient> {
        let adb_path = self.adb_path.unwrap_or_else(AdbClient::find_adb);
        Ok(AdbClient {
            adb_path,
            timeout: self.timeout,
        })
    }
}
