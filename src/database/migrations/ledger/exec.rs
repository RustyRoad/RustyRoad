//! Backend-dispatching statement execution for the ledger.
//!
//! These helpers bind parameters once and run against whichever pool the
//! connection holds, so the rest of the ledger never repeats a per-backend match.

use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;

/// Binds `binds` in order onto a query built by `$build`, then awaits `$finish`
/// and normalizes each backend's distinct result type via `$map`.
macro_rules! dispatch {
    ($connection:expr, $build:expr, $binds:expr, $finish:ident, $map:expr) => {{
        macro_rules! run {
            ($pool:expr) => {{
                let mut query = $build;
                for bind in $binds {
                    query = query.bind(*bind);
                }
                query.$finish($pool.as_ref()).await.map($map)
            }};
        }
        match $connection {
            DatabaseConnection::Pg(pool) => run!(pool),
            DatabaseConnection::MySql(pool) => run!(pool),
            DatabaseConnection::Sqlite(pool) => run!(pool),
        }
    }};
}

/// Runs a statement with the given string parameters bound in order.
pub(super) async fn execute(
    connection: &DatabaseConnection,
    sql: &str,
    binds: &[&str],
) -> Result<(), CustomMigrationError> {
    dispatch!(
        connection,
        sqlx::query(sqlx::AssertSqlSafe(sql.to_owned())),
        binds,
        execute,
        |_| ()
    )
    .map_err(CustomMigrationError::SqlxError)
}

/// Returns `true` when the query yields at least one row.
pub(super) async fn exists(
    connection: &DatabaseConnection,
    sql: &str,
    binds: &[&str],
) -> Result<bool, CustomMigrationError> {
    dispatch!(
        connection,
        sqlx::query_scalar::<_, i32>(sqlx::AssertSqlSafe(sql.to_owned())),
        binds,
        fetch_optional,
        |row: Option<i32>| row.is_some()
    )
    .map_err(CustomMigrationError::SqlxError)
}
