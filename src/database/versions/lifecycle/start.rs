//! Starting a migration: apply operations, backfill, then publish a version.

use super::apply;
use crate::database::migrations::CustomMigrationError;
use crate::database::versions::ops::{model::Migration, plan};
use crate::database::versions::{history, naming, query, record};
use crate::database::DatabaseConnection;

/// Outcome of starting a migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Started {
    /// Version now being served alongside the previous one.
    pub version: String,
    /// Schema name clients set on `search_path` to select it.
    pub schema: String,
    /// Tables backfilled during the start phase.
    pub backfilled: Vec<String>,
}

/// Applies `migration`, then publishes `version` as a new schema serving the result.
///
/// Fails when a migration is already in progress: only one may be active, so the
/// version chain cannot fork.
pub async fn start(
    connection: &DatabaseConnection,
    schema: &str,
    version: &str,
    migration: &Migration,
) -> Result<Started, CustomMigrationError> {
    history::ensure_table(connection).await?;

    if let Some(active) = query::active(connection).await? {
        return Err(super::conflict(format!(
            "migration '{active}' is already in progress; complete or roll it back first"
        )));
    }

    let version_schema = naming::versioned_schema(schema, version);
    let plan = plan(migration, &version_schema);

    record::start(connection, version).await?;

    match apply::apply(connection, schema, version, &plan).await {
        Ok(backfilled) => Ok(Started {
            version: version.to_string(),
            schema: version_schema,
            backfilled,
        }),
        Err(error) => {
            // Withdraw the history row so a failed start does not block the next one.
            record::discard(connection, version).await?;
            Err(error)
        }
    }
}
