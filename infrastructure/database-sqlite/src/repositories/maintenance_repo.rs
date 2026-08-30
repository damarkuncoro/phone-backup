use std::collections::HashSet;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use ports::MaintenanceRepositoryPort;

pub struct MaintenanceRepository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl MaintenanceRepository {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }
}

impl MaintenanceRepositoryPort for MaintenanceRepository {
    fn get_all_referenced_hashes(&self) -> anyhow::Result<HashSet<String>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT hash_sha256 FROM files WHERE hash_sha256 IS NOT NULL
             UNION
             SELECT chunk_hash FROM file_chunks"
        )?;

        let hash_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut hashes = HashSet::new();
        for h in hash_iter {
            hashes.insert(h?);
        }

        // Add hashes from structured data manually to be safe
        let mut stmt_data = conn.prepare("SELECT object_id FROM snapshot_data")?;
        let data_iter = stmt_data.query_map([], |row| row.get::<_, String>(0))?;
        for path in data_iter {
            let path = path?;
            if let Some(filename) = path.split('/').last() {
                let hash = filename.split('.').next().unwrap_or("");
                if !hash.is_empty() {
                    hashes.insert(hash.to_string());
                }
            }
        }

        Ok(hashes)
    }

    fn optimize(&self) -> anyhow::Result<()> {
        let conn = self.pool.get()?;
        conn.execute_batch(
            "PRAGMA optimize;
             VACUUM;
             ANALYZE;"
        )?;
        Ok(())
    }
}
