//! Applying a plan during the start phase.

use super::publish;
use crate::database::migrations::CustomMigrationError;
use crate::database::versions::ops::{backfill, Plan};
use crate::database::versions::{exec, stored_plan};
use crate::database::DatabaseConnection;

/// Applies the plan's statements, runs backfills, and publishes the version.
pub(super) async fn apply(
    connection: &DatabaseConnection,
    schema: &str,
    version: &str,
    plan: &Plan,
) -> Result<Vec<String>, CustomMigrationError> {
    for statement in &plan.statements {
        exec::run(connection, statement, &[]).await?;
    }

    // Triggers only fire on write, so rows never touched need an explicit pass.
    let mut backfilled = Vec::new();
    for table in &plan.backfill_tables {
        backfill::run(connection, table, backfill::DEFAULT_BATCH_SIZE).await?;
        backfilled.push(table.clone());
    }

    publish::publish(connection, schema, version).await?;

    // `complete` runs in a later invocation and needs this plan.
    stored_plan::save(connection, version, plan).await?;

    Ok(backfilled)
}
