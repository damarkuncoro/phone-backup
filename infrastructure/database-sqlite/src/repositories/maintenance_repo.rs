use rusqlite::Connection;
use std::collections::HashSet;

pub struct MaintenanceRepository;

impl MaintenanceRepository {
    pub fn get_all_referenced_hashes(conn: &Connection) -> anyhow::Result<HashSet<String>> {
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
}
