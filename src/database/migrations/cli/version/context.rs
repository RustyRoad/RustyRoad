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

/// Reads a migration's up.sql, or reports the failure and exits.
pub(super) fn up_sql(version: &str) -> Vec<String> {
    match source::up(version) {
        Ok(sql) => sql,
        Err(error) => {
            super::report::fail(&error.to_string());
            unreachable!("fail exits the process")
        }
    }
}
