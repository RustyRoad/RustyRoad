//! Lifecycle state collection for `version-status`.

use crate::database::migrations::CustomMigrationError;
use crate::database::versions::{history, lifecycle, query};
use crate::database::DatabaseConnection;

/// Current lifecycle state.
pub(super) struct Status {
    pub active: Option<String>,
    pub current: Option<String>,
    pub baseline: Option<String>,
    pub versioned: bool,
    pub schema: String,
}

/// Reads the current lifecycle state.
pub(super) async fn collect(
    connection: &DatabaseConnection,
    schema: &str,
) -> Result<Status, CustomMigrationError> {
    history::ensure_table(connection).await?;

    Ok(Status {
        active: query::active(connection).await?,
        current: query::current_version(connection).await?,
        baseline: query::latest_baseline(connection).await?,
        versioned: lifecycle::supports_versions(connection),
        schema: schema.to_string(),
    })
}
