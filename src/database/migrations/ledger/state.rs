//! Reading migration state from the ledger.
//!
//! This is the read that makes `rustyroad migration all` idempotent.

use super::{dialect, exec, identity, DIRECTION_UP};
use crate::database::migrations::{CustomMigrationError, MigrationDirection};
use crate::database::DatabaseConnection;

/// Returns `true` when a row records `migration_name` in `direction`.
async fn has_row(
    connection: &DatabaseConnection,
    migration_name: &str,
    direction: &str,
) -> Result<bool, CustomMigrationError> {
    let sql = dialect::pick(
        connection,
        "SELECT 1 FROM _rustyroad_migrations WHERE name = $1 AND direction = $2 LIMIT 1",
        "SELECT 1 FROM _rustyroad_migrations WHERE name = ? AND direction = ? LIMIT 1",
    );

    exec::exists(connection, sql, &[migration_name, direction]).await
}

/// Returns `true` when the migration has an authoritative `up` ledger row.
///
/// This is an execution/idempotency gate, not proof of SQL execution or effects.
///
/// State is decided by the presence of an `up` row rather than by row ordering: the
/// upsert in [`super::record`] reuses a row's surrogate key, so `id` order does not
/// track recency. Recording keeps at most one direction row per migration, which
/// makes presence authoritative.
///
/// Accepts a full `<timestamp>-<name>` identity and also consults the legacy
/// bare-name entry. Ledgers written by earlier versions stored bare names, so
/// without that second lookup, switching to full-directory-name recording would
/// make every historically applied migration look unapplied and replay the whole
/// history on the next run.
pub async fn is_applied(
    connection: &DatabaseConnection,
    migration_name: &str,
) -> Result<bool, CustomMigrationError> {
    if has_row(connection, migration_name, DIRECTION_UP).await? {
        return Ok(true);
    }
    if has_row(connection, migration_name, super::DIRECTION_DOWN).await? {
        return Ok(false);
    }

    // Not recorded under the full identity: consult the legacy bare-name entry.
    match identity::legacy_bare_name(migration_name) {
        Some(bare) => has_row(connection, bare, DIRECTION_UP).await,
        None => Ok(false),
    }
}

/// Returns `true` when running `direction` would be skipped according to the ledger.
pub async fn should_skip(
    connection: &DatabaseConnection,
    migration_name: &str,
    direction: MigrationDirection,
) -> Result<bool, CustomMigrationError> {
    let applied = is_applied(connection, migration_name).await?;

    Ok(match direction {
        MigrationDirection::Up => applied,
        MigrationDirection::Down => !applied,
    })
}
