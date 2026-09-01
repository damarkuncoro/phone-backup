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
        let mut hashes = HashSet::new();

        // 1. Get all storage keys from physical objects
        let mut stmt = conn.prepare("SELECT storage_key FROM chunk_objects")?;
        let key_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;
        for k in key_iter {
            hashes.insert(k?);
        }

        // 2. Get all manifest paths
        let mut stmt_snap = conn.prepare("SELECT id FROM snapshots")?;
        let snap_iter = stmt_snap.query_map([], |row| row.get::<_, String>(0))?;
        for id in snap_iter {
            hashes.insert(format!("manifests/{}.json", id?));
        }

        // 3. (Legacy) hash_sha256 from files if still used for anything in storage
        let mut stmt_files = conn.prepare("SELECT hash_sha256 FROM files WHERE hash_sha256 IS NOT NULL")?;
        let file_hash_iter = stmt_files.query_map([], |row| row.get::<_, String>(0))?;
        for h in file_hash_iter {
            hashes.insert(h?);
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
        total_deleted += tx.execute(
            "DELETE FROM files
             WHERE id NOT IN (SELECT file_id FROM snapshot_files)",
            []
        )? as u64;

        // 3. Delete logical chunks not linked to any file
        total_deleted += tx.execute(
            "DELETE FROM chunks
             WHERE id NOT IN (SELECT chunk_id FROM file_chunks)",
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
