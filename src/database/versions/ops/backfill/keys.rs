//! Primary key discovery for the backfill loop.

use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;
use sqlx::Row as _;

/// Reads the primary key columns of `table`, in key order.
const PRIMARY_KEY_SQL: &str = "SELECT a.attname \
     FROM pg_index i \
     JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey) \
     WHERE i.indrelid = $1::regclass AND i.indisprimary \
     ORDER BY array_position(i.indkey, a.attnum)";

/// Returns the primary key columns of `table`.
///
/// The backfill paginates over the primary key, so a table without one cannot be
/// backfilled in batches; that is reported rather than silently falling back to a
/// full-table update, which would hold locks for the duration.
pub(super) async fn primary_key(
    connection: &DatabaseConnection,
    table: &str,
) -> Result<Vec<String>, CustomMigrationError> {
    let DatabaseConnection::Pg(pool) = connection else {
        return Ok(Vec::new());
    };

    let rows = sqlx::query(PRIMARY_KEY_SQL)
        .bind(table)
        .fetch_all(pool.as_ref())
        .await
        .map_err(CustomMigrationError::SqlxError)?;

    Ok(rows
        .iter()
        .filter_map(|row| row.try_get::<String, _>("attname").ok())
        .collect())
}
