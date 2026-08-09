//! Baseline: adopt an existing database's current schema as the starting point.
//!
//! Modelled on `pgroll baseline`, which inserts a completed migration record
//! capturing current schema state so that history before it is no longer replayed.
//!
//! RustyRoad's equivalent records every migration currently on disk as baselined,
//! without executing any of it. This is the operation needed when a database's
//! schema is already current but its ledger does not say so — replaying that
//! history would fail against columns that later migrations already superseded.

use super::{ledger, migration_directories, CustomMigrationError};
use crate::database::{Database, DatabaseConnection};
use std::io;
use std::path::Path;

/// Directory holding migration folders.
const MIGRATIONS_DIR: &str = "./config/database/migrations";

/// Records every on-disk migration as baselined without running it.
///
/// Returns the ledger identities that were newly recorded.
pub async fn create(connection: &DatabaseConnection) -> Result<Vec<String>, CustomMigrationError> {
    record_ids(connection, &identities()?).await
}

/// Records each identity in `ids` as baselined, skipping those already recorded.
///
/// No migration SQL is executed. This asserts the database already reflects the
/// changes, which is the whole point: replaying them would fail against a schema
/// that later migrations already superseded.
pub async fn record_ids(
    connection: &DatabaseConnection,
    ids: &[String],
) -> Result<Vec<String>, CustomMigrationError> {
    ledger::ensure_table(connection).await?;

    let mut recorded = Vec::new();
    for id in ids {
        if ledger::is_applied(connection, id).await? {
            continue;
        }
        let checksum =
            ledger::file_checksum(&Path::new(MIGRATIONS_DIR).join(id).join("up.sql")).ok();
        ledger::record_with_metadata(
            connection,
            id,
            super::MigrationDirection::Up,
            ledger::Provenance::Baselined,
            checksum.as_deref(),
        )
        .await?;
        recorded.push(id.clone());
    }
    Ok(recorded)
}

/// Returns the ledger identity of every migration directory, in apply order.
pub fn identities() -> Result<Vec<String>, CustomMigrationError> {
    let root = Path::new(MIGRATIONS_DIR);
    if !root.exists() {
        return Err(CustomMigrationError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No migrations directory found at '{MIGRATIONS_DIR}'."),
        )));
    }

    let mut directories = migration_directories(root).map_err(CustomMigrationError::IoError)?;
    directories.sort();

    Ok(directories
        .iter()
        .filter_map(|path| path.file_name()?.to_str().map(str::to_string))
        .collect())
}

/// Opens a connection using the configured environment.
pub async fn connect() -> Result<DatabaseConnection, CustomMigrationError> {
    let database = Database::get_database_from_rustyroad_toml().map_err(|error| {
        CustomMigrationError::IoError(io::Error::other(format!(
            "Couldn't parse the rustyroad.toml file: {error}"
        )))
    })?;

    Database::create_database_connection(&database)
        .await
        .map_err(CustomMigrationError::SendError)
}
