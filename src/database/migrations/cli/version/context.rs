//! Connecting and reading migration SQL for lifecycle commands.

use crate::database::migrations::baseline;
use crate::database::versions::source;
use crate::database::DatabaseConnection;

/// Opens a connection, or reports the failure and exits.
pub(super) async fn connect() -> DatabaseConnection {
    match baseline::connect().await {
        Ok(connection) => connection,
        Err(error) => {
            super::report::fail(&error.to_string());
            unreachable!("fail exits the process")
        }
    }
}

/// Loads a migration definition, or reports the failure and exits.
pub(super) fn migration(version: &str) -> crate::database::versions::ops::model::Migration {
    match source::load(version) {
        Ok(migration) => migration,
        Err(error) => {
            super::report::fail(&error.to_string());
            unreachable!("fail exits the process")
        }
    }
}
