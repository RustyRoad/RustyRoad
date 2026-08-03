//! Reading a single history row.

use super::field;
use crate::database::migrations::CustomMigrationError;
use crate::database::versions::history::Entry;
use crate::database::DatabaseConnection;

/// Returns the recorded entry for `name`, if present.
pub async fn entry(
    connection: &DatabaseConnection,
    name: &str,
) -> Result<Option<Entry>, CustomMigrationError> {
    // Concatenating a NULL parent would yield NULL on Postgres, so each field is
    // read independently rather than packed by the database.
    let Some(done) = field::read(connection, name, "CAST(done AS TEXT)").await? else {
        return Ok(None);
    };

    Ok(Some(Entry {
        name: name.to_string(),
        parent: super::parent_of(connection, name).await?,
        done: is_true(&done),
        migration_type: field::read(connection, name, "migration_type")
            .await?
            .unwrap_or_default(),
    }))
}

/// Interprets a boolean rendered as text by any supported backend.
fn is_true(value: &str) -> bool {
    matches!(value, "true" | "t" | "1" | "TRUE")
}
