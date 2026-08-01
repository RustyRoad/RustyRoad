//! Tests for restricting breaking-change scanning to pending migrations.

use super::pending;
use crate::database::migrations::{ledger, MigrationDirection};
use crate::database::DatabaseConnection;
use std::path::PathBuf;
use std::sync::Arc;

/// Builds an in-memory SQLite ledger.
async fn sqlite_ledger() -> DatabaseConnection {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to open in-memory sqlite database");
    let connection = DatabaseConnection::Sqlite(Arc::new(pool));
    ledger::ensure_table(&connection)
        .await
        .expect("failed to create ledger table");
    connection
}

#[tokio::test]
async fn applied_migrations_are_excluded() {
    let connection = sqlite_ledger().await;
    let applied = "20251114211514-add_targeting_metadata_columns";
    let pending_id = "20251120000000-zone_id_nullable";

    ledger::record(&connection, applied, MigrationDirection::Up)
        .await
        .unwrap();

    let directories = vec![
        PathBuf::from(format!("./config/database/migrations/{applied}")),
        PathBuf::from(format!("./config/database/migrations/{pending_id}")),
    ];

    // The scanner must not warn about (and therefore block on) a migration the
    // ledger already records, because applying it would be a no-op.
    let result = pending::filter(&connection, directories).await;

    assert_eq!(result.len(), 1);
    assert!(result[0].ends_with(pending_id));
}

#[tokio::test]
async fn legacy_bare_name_entries_are_excluded() {
    let connection = sqlite_ledger().await;

    // A ledger written by an older version stored bare names.
    ledger::record(&connection, "add_targeting_metadata_columns", MigrationDirection::Up)
        .await
        .unwrap();

    let directories = vec![PathBuf::from(
        "./config/database/migrations/20251114211514-add_targeting_metadata_columns",
    )];

    assert!(pending::filter(&connection, directories).await.is_empty());
}

#[tokio::test]
async fn unrecorded_migrations_are_kept() {
    let connection = sqlite_ledger().await;
    let directories = vec![PathBuf::from(
        "./config/database/migrations/20251120000000-zone_id_nullable",
    )];

    assert_eq!(pending::filter(&connection, directories.clone()).await, directories);
}
