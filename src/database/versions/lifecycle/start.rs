//! Starting a migration: apply DDL, then publish a new schema version.

use super::publish;
use crate::database::migrations::CustomMigrationError;
use crate::database::versions::{exec, history, naming, query, record};
use crate::database::DatabaseConnection;

/// Outcome of starting a migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Started {
    /// Version now being served alongside the previous one.
    pub version: String,
    /// Schema name clients set on `search_path` to select it.
    pub schema: String,
}

/// Applies `sql`, then publishes `version` as a new schema serving the result.
///
/// Fails when a migration is already in progress: only one may be active, so the
/// version chain cannot fork.
pub async fn start(
    connection: &DatabaseConnection,
    schema: &str,
    version: &str,
    sql: &[String],
) -> Result<Started, CustomMigrationError> {
    history::ensure_table(connection).await?;

    if let Some(active) = query::active(connection).await? {
        return Err(super::conflict(format!(
            "migration '{active}' is already in progress; complete or roll it back first"
        )));
    }

    record::start(connection, version).await?;
    apply(connection, schema, version, sql).await?;

    Ok(Started {
        version: version.to_string(),
        schema: naming::versioned_schema(schema, version),
    })
}

/// Applies the migration DDL and publishes its views, withdrawing the history row
/// if anything fails so the next migration is not blocked by a dangling record.
async fn apply(
    connection: &DatabaseConnection,
    schema: &str,
    version: &str,
    sql: &[String],
) -> Result<(), CustomMigrationError> {
    for statement in sql {
        if let Err(error) = exec::run(connection, statement, &[]).await {
            record::discard(connection, version).await?;
            return Err(error);
        }
    }

    if let Err(error) = publish::publish(connection, schema, version).await {
        record::discard(connection, version).await?;
        return Err(error);
    }

    Ok(())
}
