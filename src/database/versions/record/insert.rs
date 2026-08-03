//! Inserting history rows.

use super::statement;
use crate::database::migrations::CustomMigrationError;
use crate::database::versions::exec;
use crate::database::versions::history::{TYPE_BASELINE, TYPE_MIGRATION};
use crate::database::versions::query;
use crate::database::DatabaseConnection;

/// Inserts `name` as an in-progress migration parented to the latest recorded one.
pub async fn start(
    connection: &DatabaseConnection,
    name: &str,
) -> Result<(), CustomMigrationError> {
    insert(connection, name, false, TYPE_MIGRATION).await
}

/// Inserts `name` as an already-completed migration.
///
/// `baseline` marks it as adopted rather than executed.
pub async fn insert_done(
    connection: &DatabaseConnection,
    name: &str,
    baseline: bool,
) -> Result<(), CustomMigrationError> {
    let kind = if baseline { TYPE_BASELINE } else { TYPE_MIGRATION };
    insert(connection, name, true, kind).await
}

/// Inserts a history row parented to the latest recorded migration.
async fn insert(
    connection: &DatabaseConnection,
    name: &str,
    done: bool,
    kind: &str,
) -> Result<(), CustomMigrationError> {
    match query::latest(connection).await? {
        Some(parent) => {
            let sql = statement::build(connection, done, true);
            exec::run(connection, &sql, &[name, &parent, kind]).await
        }
        None => {
            let sql = statement::build(connection, done, false);
            exec::run(connection, &sql, &[name, kind]).await
        }
    }
}
