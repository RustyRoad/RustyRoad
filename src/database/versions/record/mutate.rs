//! Updating and removing history rows.

use crate::database::migrations::CustomMigrationError;
use crate::database::versions::exec;
use crate::database::versions::history::HISTORY_TABLE;
use crate::database::DatabaseConnection;

/// Marks a migration as completed.
pub async fn complete(
    connection: &DatabaseConnection,
    name: &str,
) -> Result<(), CustomMigrationError> {
    let sql = exec::pick(
        connection,
        &format!(
            "UPDATE {HISTORY_TABLE} SET done = TRUE, updated_at = CURRENT_TIMESTAMP \
             WHERE name = $1"
        ),
        &format!(
            "UPDATE {HISTORY_TABLE} SET done = TRUE, updated_at = CURRENT_TIMESTAMP \
             WHERE name = ?"
        ),
    )
    .to_string();
    exec::run(connection, &sql, &[name]).await
}

/// Removes an in-progress migration, abandoning it.
///
/// Completed migrations are never removed by this path.
pub async fn discard(
    connection: &DatabaseConnection,
    name: &str,
) -> Result<(), CustomMigrationError> {
    let sql = exec::pick(
        connection,
        &format!("DELETE FROM {HISTORY_TABLE} WHERE name = $1 AND done = FALSE"),
        &format!("DELETE FROM {HISTORY_TABLE} WHERE name = ? AND done = FALSE"),
    )
    .to_string();
    exec::run(connection, &sql, &[name]).await
}
