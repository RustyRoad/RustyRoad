//! Reading a single expression from a history row.

use crate::database::migrations::CustomMigrationError;
use crate::database::versions::exec;
use crate::database::versions::history::HISTORY_TABLE;
use crate::database::DatabaseConnection;

/// Reads `expression` from the row named `name`.
pub(super) async fn read(
    connection: &DatabaseConnection,
    name: &str,
    expression: &str,
) -> Result<Option<String>, CustomMigrationError> {
    let sql = exec::pick(
        connection,
        &format!("SELECT {expression} FROM {HISTORY_TABLE} WHERE name = $1"),
        &format!("SELECT {expression} FROM {HISTORY_TABLE} WHERE name = ?"),
    )
    .to_string();
    exec::scalar(connection, &sql, &[name]).await
}
