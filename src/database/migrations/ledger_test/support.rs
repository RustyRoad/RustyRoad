//! Shared fixture for ledger tests.
//!
//! Tests run against a real in-memory SQLite database rather than a mock, so they
//! exercise the actual SQL and parameter binding.

use std::sync::Arc;

use crate::database::migrations::ledger;
use crate::database::DatabaseConnection;

/// Builds an in-memory SQLite connection with the ledger table created.
pub(super) async fn sqlite_ledger() -> DatabaseConnection {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to open in-memory sqlite database");
    let connection = DatabaseConnection::Sqlite(Arc::new(pool));
    ledger::ensure_table(&connection)
        .await
        .expect("failed to create ledger table");
    connection
}

/// Counts ledger rows recorded for `name`.
pub(super) async fn row_count(connection: &DatabaseConnection, name: &str) -> i64 {
    let DatabaseConnection::Sqlite(pool) = connection else {
        panic!("expected sqlite connection");
    };
    sqlx::query_scalar("SELECT COUNT(*) FROM _rustyroad_migrations WHERE name = ?")
        .bind(name)
        .fetch_one(pool.as_ref())
        .await
        .expect("failed to count ledger rows")
}
