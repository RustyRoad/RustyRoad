//! Backend-dispatching execution for version and history statements.

use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;

/// Selects the placeholder style for `connection`.
pub fn pick<'a>(
    connection: &DatabaseConnection,
    numbered: &'a str,
    positional: &'a str,
) -> &'a str {
    match connection {
        DatabaseConnection::Pg(_) => numbered,
        DatabaseConnection::MySql(_) | DatabaseConnection::Sqlite(_) => positional,
    }
}

/// Binds `binds` in order onto a query, then awaits `$finish`, normalizing via `$map`.
macro_rules! dispatch {
    ($connection:expr, $build:expr, $binds:expr, $finish:ident, $map:expr) => {{
        macro_rules! go {
            ($pool:expr) => {{
                let mut query = $build;
                for bind in $binds {
                    query = query.bind(*bind);
                }
                query.$finish($pool.as_ref()).await.map($map)
            }};
        }
        match $connection {
            DatabaseConnection::Pg(pool) => go!(pool),
            DatabaseConnection::MySql(pool) => go!(pool),
            DatabaseConnection::Sqlite(pool) => go!(pool),
        }
    }};
}

/// Runs a statement with the given parameters bound in order.
pub async fn run(
    connection: &DatabaseConnection,
    sql: &str,
    binds: &[&str],
) -> Result<(), CustomMigrationError> {
    dispatch!(connection, sqlx::query(sql), binds, execute, |_| ())
        .map_err(CustomMigrationError::SqlxError)
}

/// Returns the first column of the first row, if any.
pub async fn scalar(
    connection: &DatabaseConnection,
    sql: &str,
    binds: &[&str],
) -> Result<Option<String>, CustomMigrationError> {
    dispatch!(
        connection,
        sqlx::query_scalar::<_, String>(sql),
        binds,
        fetch_optional,
        |row| row
    )
    .map_err(CustomMigrationError::SqlxError)
}
