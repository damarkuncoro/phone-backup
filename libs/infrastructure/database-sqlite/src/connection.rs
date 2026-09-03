use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::sync::Arc;

use super::facade::SqliteRepository;
use super::schema;

/// Custom connection initializer to ensure PRAGMAs are set for every connection in the pool
#[derive(Debug)]
pub struct SqliteCustomizer {
    pub passphrase: Option<String>,
}

impl r2d2::CustomizeConnection<Connection, rusqlite::Error> for SqliteCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), rusqlite::Error> {
        if let Some(ref pwd) = self.passphrase {
            let escaped = pwd.replace('\'', "''");
            let _ = conn.execute(&format!("PRAGMA key = '{}';", escaped), []);
        }
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 5000;",
        )
    }
}

/// FACTORY: Pusat pembuatan repositori
pub struct SqliteRepositoryFactory;

impl SqliteRepositoryFactory {
    pub fn create_default(path: &str) -> anyhow::Result<SqliteRepository> {
        SqliteRepository::builder()
            .with_path(path)
            .run_migrations()
            .build()
    }

    pub fn create_encrypted(path: &str, passphrase: &str) -> anyhow::Result<SqliteRepository> {
        SqliteRepository::builder()
            .with_path(path)
            .with_passphrase(passphrase)
            .run_migrations()
            .build()
    }
}

/// BUILDER: Konfigurasi fleksibel untuk SqliteRepository
#[derive(Default)]
pub struct SqliteRepositoryBuilder {
    path: Option<String>,
    passphrase: Option<String>,
    run_migrations: bool,
}

impl SqliteRepositoryBuilder {
    pub fn new() -> Self {
        Self {
            path: None,
            passphrase: None,
            run_migrations: false,
        }
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub fn with_passphrase(mut self, passphrase: &str) -> Self {
        self.passphrase = Some(passphrase.to_string());
        self
    }

    pub fn run_migrations(mut self) -> Self {
        self.run_migrations = true;
        self
    }

    pub fn build(self) -> anyhow::Result<SqliteRepository> {
        let path = self
            .path
            .ok_or_else(|| anyhow::anyhow!("Database path is required"))?;

        let manager = SqliteConnectionManager::file(&path);
        let customizer = SqliteCustomizer {
            passphrase: self.passphrase,
        };
        let pool = Pool::builder()
            .connection_customizer(Box::new(customizer))
            .build(manager)?;

        if self.run_migrations {
            let conn = pool.get()?;
            schema::init_schema(&conn)?;
        }

        Ok(SqliteRepository::from_pool(Arc::new(pool)))
    }
}
