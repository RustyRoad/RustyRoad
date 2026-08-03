//! Completing and rolling back the in-progress migration.

use super::publish;
use crate::database::migrations::CustomMigrationError;
use crate::database::versions::{exec, history, query, record};
use crate::database::DatabaseConnection;

/// Finalizes the active migration and drops the previous version's schema.
pub async fn complete(
    connection: &DatabaseConnection,
    schema: &str,
) -> Result<String, CustomMigrationError> {
    history::ensure_table(connection).await?;

    let Some(active) = query::active(connection).await? else {
        return Err(super::conflict("no migration is in progress".to_string()));
    };

    // The previous version stops being served once this one is complete.
    if let Some(previous) = query::parent_of(connection, &active).await? {
        publish::unpublish(connection, schema, &previous).await?;
    }

    record::complete(connection, &active).await?;
    Ok(active)
}

/// Rolls back the active migration, dropping its versioned schema.
///
/// Only an incomplete migration can be rolled back this way; once completed, the
/// previous version's schema no longer exists to return to.
pub async fn rollback(
    connection: &DatabaseConnection,
    schema: &str,
    down_sql: &[String],
) -> Result<String, CustomMigrationError> {
    history::ensure_table(connection).await?;

    let Some(active) = query::active(connection).await? else {
        return Err(super::conflict("no migration is in progress".to_string()));
    };

    publish::unpublish(connection, schema, &active).await?;
    for statement in down_sql {
        exec::run(connection, statement, &[]).await?;
    }

    record::discard(connection, &active).await?;
    Ok(active)
}
