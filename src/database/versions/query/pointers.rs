//! Version pointers: what is active, current, and latest.

use crate::database::migrations::CustomMigrationError;
use crate::database::versions::exec;
use crate::database::versions::history::HISTORY_TABLE;
use crate::database::DatabaseConnection;

/// Returns the name of the in-progress migration, if any.
pub async fn active(
    connection: &DatabaseConnection,
) -> Result<Option<String>, CustomMigrationError> {
    let sql = format!("SELECT name FROM {HISTORY_TABLE} WHERE done = FALSE LIMIT 1");
    exec::scalar(connection, &sql, &[]).await
}

/// Returns the most recently recorded migration, completed or not.
///
/// This is the parent for the next migration inserted.
pub async fn latest(
    connection: &DatabaseConnection,
) -> Result<Option<String>, CustomMigrationError> {
    let sql = format!(
        "SELECT name FROM {HISTORY_TABLE} ORDER BY created_at DESC, name DESC LIMIT 1"
    );
    exec::scalar(connection, &sql, &[]).await
}

/// Returns the most recent completed migration, which is the version being served.
pub async fn current_version(
    connection: &DatabaseConnection,
) -> Result<Option<String>, CustomMigrationError> {
    let sql = format!(
        "SELECT name FROM {HISTORY_TABLE} WHERE done = TRUE \
         ORDER BY created_at DESC, name DESC LIMIT 1"
    );
    exec::scalar(connection, &sql, &[]).await
}
