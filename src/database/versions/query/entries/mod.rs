//! History row lookups: parents, baselines, and full entries.

mod field;
mod row;

use crate::database::migrations::CustomMigrationError;
use crate::database::versions::exec;
use crate::database::versions::history::{HISTORY_TABLE, TYPE_BASELINE};
use crate::database::DatabaseConnection;

pub use row::entry;

/// Returns the migration immediately preceding `name`.
///
/// A root migration has no parent, so `None` is returned both when `name` is
/// unknown and when its parent is NULL.
pub async fn parent_of(
    connection: &DatabaseConnection,
    name: &str,
) -> Result<Option<String>, CustomMigrationError> {
    let sql = exec::pick(
        connection,
        &format!("SELECT parent FROM {HISTORY_TABLE} WHERE name = $1 AND parent IS NOT NULL"),
        &format!("SELECT parent FROM {HISTORY_TABLE} WHERE name = ? AND parent IS NOT NULL"),
    )
    .to_string();

    // Guard against a backend coercing NULL into an empty string.
    Ok(exec::scalar(connection, &sql, &[name])
        .await?
        .filter(|parent| !parent.is_empty()))
}

/// Returns the most recent baseline, or `None` when history has never been reset.
pub async fn latest_baseline(
    connection: &DatabaseConnection,
) -> Result<Option<String>, CustomMigrationError> {
    let order = "ORDER BY created_at DESC, name DESC LIMIT 1";
    let sql = exec::pick(
        connection,
        &format!("SELECT name FROM {HISTORY_TABLE} WHERE migration_type = $1 {order}"),
        &format!("SELECT name FROM {HISTORY_TABLE} WHERE migration_type = ? {order}"),
    )
    .to_string();
    exec::scalar(connection, &sql, &[TYPE_BASELINE]).await
}
