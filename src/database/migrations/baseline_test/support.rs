//! Shared fixture for baseline tests.

use crate::database::migrations::ledger;
use crate::database::DatabaseConnection;
use std::sync::Arc;

/// Builds an in-memory SQLite ledger.
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
