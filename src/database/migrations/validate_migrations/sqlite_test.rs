use super::api::validate_with_database;
use super::test_support::{sqlite_database, write_migration};
use crate::database::DatabaseType;

#[tokio::test]
async fn complete_chain_does_not_touch_shared_database() {
    let root = tempfile::tempdir().unwrap();
    let migrations = root.path().join("migrations");
    write_migration(
        &migrations,
        "20240101-create",
        "CREATE TABLE users (id INTEGER);",
    );
    write_migration(
        &migrations,
        "20240202-alter",
        "ALTER TABLE users ADD email TEXT;",
    );
    let shared = root.path().join("shared_test_database");
    let report = validate_with_database(&sqlite_database(&shared), &migrations)
        .await
        .unwrap();
    assert_eq!(report.migrations_validated, 2);
    assert_eq!(report.database_type, DatabaseType::Sqlite);
    assert!(!shared.with_extension("db").exists());
}

#[tokio::test]
async fn failure_names_the_migration() {
    let root = tempfile::tempdir().unwrap();
    let migrations = root.path().join("migrations");
    write_migration(
        &migrations,
        "20240101-create",
        "CREATE TABLE users (id INTEGER);",
    );
    write_migration(
        &migrations,
        "20240202-broken",
        "ALTER TABLE missing ADD email TEXT;",
    );
    let error = validate_with_database(&sqlite_database(&root.path().join("shared")), &migrations)
        .await
        .unwrap_err()
        .to_string();
    assert!(error.contains("20240202-broken"));
    assert!(error.contains("isolated SQLite validation database"));
}
