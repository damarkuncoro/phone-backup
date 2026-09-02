use phone_backup_adapter_database_sqlite::SqliteRepository;

pub fn setup_test_repo() -> SqliteRepository {
    // We use a named in-memory database with a shared cache so that
    // multiple connections from the r2d2 pool see the same tables/data.
    // Each test gets a unique name to avoid interference.
    let db_name = format!(
        "file:test_db_{}?mode=memory&cache=shared",
        uuid::Uuid::new_v4()
    );

    SqliteRepository::builder()
        .with_path(&db_name)
        .run_migrations()
        .build()
        .expect("Failed to create test repository")
}
