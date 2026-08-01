//! Recording executed migrations in the ledger.

use super::{exec, sql, DIRECTION_DOWN, DIRECTION_UP};
use crate::database::migrations::{CustomMigrationError, MigrationDirection};
use crate::database::DatabaseConnection;

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
    let binds = [migration_name, direction_label(direction)];

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
