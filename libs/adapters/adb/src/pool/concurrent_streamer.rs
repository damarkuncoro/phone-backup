use super::worker_pool::AdbWorkerPool;
use anyhow::Result;
use domain::DeviceId;
use std::io::Read;

pub struct ConcurrentAdbStreamer {
    pool: AdbWorkerPool,
}

impl ConcurrentAdbStreamer {
    pub fn new(pool: AdbWorkerPool) -> Self {
        Self { pool }
    }

    pub fn stream_file(
        &self,
        device_id: &DeviceId,
        remote_path: &str,
    ) -> Result<Box<dyn Read>> {
        self.pool.client().stream_file(device_id.0.as_str(), remote_path)
    }

    pub fn pool(&self) -> &AdbWorkerPool {
        &self.pool
    }
}
