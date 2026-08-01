//! Restricts breaking-change scanning to migrations that will actually run.
//!
//! The scanner reads `up.sql` from disk and has no notion of what the target
//! database already contains. Scanning migrations the ledger already records as
//! applied blocks commands that would otherwise be no-ops, so the set of
//! directories is filtered through the ledger first.

use crate::database::migrations::ledger;
use crate::database::{Database, DatabaseConnection};
use std::path::{Path, PathBuf};

/// Returns the subset of `directories` whose migrations are not yet applied.
///
/// When the ledger cannot be consulted the input is returned unchanged, so a
/// database that is unreachable makes the scanner more cautious, never less.
pub(super) async fn unapplied(directories: Vec<PathBuf>) -> Vec<PathBuf> {
    match connect().await {
        Some(connection) => filter(&connection, directories).await,
        None => directories,
    }
}

/// Filters `directories` down to those not recorded as applied in `connection`.
pub(super) async fn filter(
    connection: &DatabaseConnection,
    directories: Vec<PathBuf>,
) -> Vec<PathBuf> {
    let mut pending = Vec::new();
    for directory in directories {
        if is_pending(connection, &directory).await {
            pending.push(directory);
        }
    }
    pending
}

/// Returns `true` when the migration still needs to be applied.
async fn is_pending(connection: &DatabaseConnection, directory: &Path) -> bool {
    let Some(path) = directory.to_str() else {
        return true;
    };
    let Some(id) = ledger::ledger_id_for_dir(path) else {
        return true;
    };

    // Treat a failed lookup as pending so scanning still happens.
    !ledger::is_applied(connection, id).await.unwrap_or(false)
}

/// Opens a ledger connection, returning `None` when unavailable.
async fn connect() -> Option<DatabaseConnection> {
    let database = Database::get_database_from_rustyroad_toml().ok()?;
    let connection = Database::create_database_connection(&database).await.ok()?;
    ledger::ensure_table(&connection).await.ok()?;
    Some(connection)
}
