//! Recording executed migrations in the ledger.

use super::{exec, sql, DIRECTION_DOWN, DIRECTION_UP};
use crate::database::migrations::{CustomMigrationError, MigrationDirection};
use crate::database::DatabaseConnection;

/// How a ledger row came to exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    /// RustyRoad successfully executed the migration SQL.
    Executed,
    /// A baseline recorded the migration without executing its SQL.
    Baselined,
}

impl Provenance {
    /// Returns the stable value persisted in the ledger.
    pub fn label(self) -> &'static str {
        match self {
            Self::Executed => "executed",
            Self::Baselined => "baselined",
        }
    }
}

/// Returns the string persisted in the ledger for `direction`.
pub fn direction_label(direction: MigrationDirection) -> &'static str {
    match direction {
        MigrationDirection::Up => DIRECTION_UP,
        MigrationDirection::Down => DIRECTION_DOWN,
    }
}

/// Records that `migration_name` was executed in `direction`.
///
/// Upserts on `(name, direction)` so the ledger holds one authoritative row per
/// migration per direction instead of accumulating a duplicate on every run, then
/// clears the opposite direction so current state is unambiguous.
///
/// Migration names are always bound, never interpolated into SQL.
pub async fn record(
    connection: &DatabaseConnection,
    migration_name: &str,
    direction: MigrationDirection,
) -> Result<(), CustomMigrationError> {
    record_with_metadata(
        connection,
        migration_name,
        direction,
        Provenance::Executed,
        None,
    )
    .await
}

/// Records a migration together with how it entered the ledger and its source digest.
pub async fn record_with_metadata(
    connection: &DatabaseConnection,
    migration_name: &str,
    direction: MigrationDirection,
    provenance: Provenance,
    checksum: Option<&str>,
) -> Result<(), CustomMigrationError> {
    let binds = [
        migration_name,
        direction_label(direction),
        provenance.label(),
        checksum.unwrap_or(""),
    ];

    if exec::execute(connection, sql::upsert(connection), &binds)
        .await
        .is_err()
    {
        // A legacy table may lack the unique constraint, making the upsert clause
        // invalid. Use a plain parameterized insert so recording still works.
        exec::execute(connection, sql::plain_insert(connection), &binds).await?;
    }

    clear_opposite(connection, migration_name, direction).await
}

/// Removes the row for the direction opposite to `direction`.
///
/// Keeping one direction row per migration is what lets [`super::is_applied`] treat
/// row presence as authoritative without depending on insertion order.
async fn clear_opposite(
    connection: &DatabaseConnection,
    migration_name: &str,
    direction: MigrationDirection,
) -> Result<(), CustomMigrationError> {
    let opposite = match direction {
        MigrationDirection::Up => DIRECTION_DOWN,
        MigrationDirection::Down => DIRECTION_UP,
    };

    exec::execute(
        connection,
        sql::delete_direction(connection),
        &[migration_name, opposite],
    )
    .await
}
