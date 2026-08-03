//! Publishing a schema version as views over the physical tables.

use crate::database::migrations::CustomMigrationError;
use crate::database::versions::{exec, introspect, view};
use crate::database::DatabaseConnection;

/// Returns `true` when the backend supports versioned schemas.
///
/// Versioned schemas rely on Postgres schema namespaces and views; MySQL and
/// SQLite have no equivalent, so their migrations run without version publishing.
pub fn supports_versions(connection: &DatabaseConnection) -> bool {
    matches!(connection, DatabaseConnection::Pg(_))
}

/// Creates the versioned schema and its views from the current physical schema.
pub(super) async fn publish(
    connection: &DatabaseConnection,
    schema: &str,
    version: &str,
) -> Result<(), CustomMigrationError> {
    if !supports_versions(connection) {
        return Ok(());
    }

    let snapshot = introspect::read_schema(connection, schema).await?;
    if snapshot.tables.is_empty() {
        // Nothing to project yet.
        return Ok(());
    }

    let major = introspect::major_version(connection).await;
    for statement in view::create_version(schema, version, &snapshot, major) {
        exec::run(connection, &statement, &[]).await?;
    }
    Ok(())
}

/// Drops the schema serving `version`, when the backend has one.
pub(super) async fn unpublish(
    connection: &DatabaseConnection,
    schema: &str,
    version: &str,
) -> Result<(), CustomMigrationError> {
    if !supports_versions(connection) {
        return Ok(());
    }
    exec::run(connection, &view::drop_schema(schema, version), &[]).await
}
