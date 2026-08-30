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

    fn prune_orphans(&self) -> anyhow::Result<u64> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        let mut total_deleted = 0;

        // 1. Delete contact objects not linked to any snapshot
        total_deleted += tx.execute(
            "DELETE FROM contact_objects
             WHERE id NOT IN (SELECT contact_id FROM snapshot_contacts)",
            []
        )? as u64;

        // 2. Delete files not linked to any snapshot
        // (Optional: depending on your logic, some files might be device-level, but usually we link them)
        total_deleted += tx.execute(
            "DELETE FROM files
             WHERE id NOT IN (SELECT file_id FROM snapshot_files)",
            []
        )? as u64;

        tx.commit()?;
        Ok(total_deleted)
    }

    fn create_database_backup(&self, destination_path: &str) -> anyhow::Result<()> {
        let src_conn = self.pool.get()?;
        let mut dst_conn = rusqlite::Connection::open(destination_path)?;

        let backup = rusqlite::backup::Backup::new(&src_conn, &mut dst_conn)?;
        backup.run_to_completion(5, std::time::Duration::from_millis(100), None)?;

        Ok(())
    }
}
