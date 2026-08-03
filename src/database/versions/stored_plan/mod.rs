//! Persisting a version's completion plan.
//!
//! `complete` runs in a separate process invocation from `start`, so the work it must
//! finish — promoting shadow columns, dropping originals, applying deferred
//! constraints — is stored alongside the history row rather than recomputed.

mod codec;

use super::exec;
use super::history::HISTORY_TABLE;
use super::ops::Plan;
use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;

/// Adds the plan column to the history table when absent.
async fn ensure_column(connection: &DatabaseConnection) -> Result<(), CustomMigrationError> {
    // Fails harmlessly when the column already exists.
    let _ = exec::run(
        connection,
        &format!("ALTER TABLE {HISTORY_TABLE} ADD COLUMN plan TEXT"),
        &[],
    )
    .await;
    Ok(())
}

/// Stores `plan` against the history row for `version`.
pub async fn save(
    connection: &DatabaseConnection,
    version: &str,
    plan: &Plan,
) -> Result<(), CustomMigrationError> {
    ensure_column(connection).await?;
    let encoded = codec::encode(plan);

    let sql = exec::pick(
        connection,
        &format!("UPDATE {HISTORY_TABLE} SET plan = $2 WHERE name = $1"),
        &format!("UPDATE {HISTORY_TABLE} SET plan = ? WHERE name = ?"),
    )
    .to_string();

    // Numbered placeholders bind name first; positional ones bind in written order.
    let binds: Vec<&str> = match connection {
        DatabaseConnection::Pg(_) => vec![version, &encoded],
        _ => vec![&encoded, version],
    };
    exec::run(connection, &sql, &binds).await
}

/// Loads the stored plan for `version`, if any.
pub async fn load(
    connection: &DatabaseConnection,
    version: &str,
) -> Result<Option<Plan>, CustomMigrationError> {
    ensure_column(connection).await?;

    let sql = exec::pick(
        connection,
        &format!("SELECT plan FROM {HISTORY_TABLE} WHERE name = $1 AND plan IS NOT NULL"),
        &format!("SELECT plan FROM {HISTORY_TABLE} WHERE name = ? AND plan IS NOT NULL"),
    )
    .to_string();

    Ok(exec::scalar(connection, &sql, &[version])
        .await?
        .filter(|encoded| !encoded.is_empty())
        .and_then(|encoded| codec::decode(&encoded)))
}
