//! Reads the current physical schema from Postgres.
//!
//! Version views are projected from these results, so internal in-flight columns
//! are mapped back to their logical names and columns pending deletion are hidden.

mod build;
mod rules;

use super::model::Schema;
use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;
use sqlx::Row as _;

/// Columns of the base tables in a schema, in declaration order.
const COLUMNS_SQL: &str = "SELECT c.table_name, c.column_name, c.column_default \
     FROM information_schema.columns c \
     JOIN information_schema.tables t \
       ON t.table_schema = c.table_schema AND t.table_name = c.table_name \
     WHERE c.table_schema = $1 AND t.table_type = 'BASE TABLE' \
     ORDER BY c.table_name, c.ordinal_position";

/// Reads the current schema, returning the logical view of each table.
///
/// `renames` are applied to logical column names so the new version's views expose
/// the renamed column immediately, rather than only after completion.
///
/// Only Postgres exposes versioned schemas; other backends return an empty
/// snapshot rather than failing, so callers can degrade cleanly.
pub async fn read_schema(
    connection: &DatabaseConnection,
    schema: &str,
    renames: &[(String, String, String)],
) -> Result<Schema, CustomMigrationError> {
    let DatabaseConnection::Pg(pool) = connection else {
        return Ok(Schema::default());
    };

    let rows = sqlx::query(COLUMNS_SQL)
        .bind(schema)
        .fetch_all(pool.as_ref())
        .await
        .map_err(CustomMigrationError::SqlxError)?;

    let parsed = rows
        .into_iter()
        .map(|row| build::Row {
            table: row.try_get("table_name").unwrap_or_default(),
            column: row.try_get("column_name").unwrap_or_default(),
            default: row.try_get("column_default").ok().flatten(),
        })
        .collect();

    Ok(build::build(parsed, renames))
}

/// Returns the server's major version, defaulting to 14 when unavailable.
pub async fn major_version(connection: &DatabaseConnection) -> u32 {
    let DatabaseConnection::Pg(pool) = connection else {
        return 0;
    };

    sqlx::query_scalar::<_, i32>("SELECT current_setting('server_version_num')::int / 10000")
        .fetch_one(pool.as_ref())
        .await
        .map(|value| value.max(0) as u32)
        .unwrap_or(14)
}
