//! Shared fixture for versioned schema tests.

use crate::database::versions::history;
use crate::database::versions::ops::model::{Migration, Operation, RawSql};
use crate::database::DatabaseConnection;
use std::sync::Arc;

/// Builds an in-memory SQLite connection with the history table created.
pub(super) async fn sqlite() -> DatabaseConnection {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to open in-memory sqlite database");
    let connection = DatabaseConnection::Sqlite(Arc::new(pool));
    history::ensure_table(&connection)
        .await
        .expect("failed to create history table");
    connection
}

/// Counts rows in the history table.
pub(super) async fn history_rows(connection: &DatabaseConnection) -> i64 {
    let DatabaseConnection::Sqlite(pool) = connection else {
        panic!("expected sqlite connection");
    };
    sqlx::query_scalar("SELECT COUNT(*) FROM _rustyroad_history")
        .fetch_one(pool.as_ref())
        .await
        .expect("failed to count history rows")
}

/// A migration with no operations.
pub(super) fn empty_migration() -> Migration {
    Migration {
        name: String::new(),
        operations: Vec::new(),
    }
}

/// A migration applying one raw SQL statement.
pub(super) fn sql_migration(sql: &str) -> Migration {
    Migration {
        name: String::new(),
        operations: vec![Operation::Sql(RawSql {
            up: sql.to_string(),
            down: None,
        })],
    }
}
