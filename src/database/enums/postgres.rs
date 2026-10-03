//! Postgres enum catalog reader.

use super::model::DatabaseEnum;
use crate::database::introspection;
use crate::database::migrations::CustomMigrationError;
use sqlx::Row;

/// Postgres enum types in the public schema, values in declaration order.
///
/// Uses the same catalog query as code generation, so the CLI reports the
/// same enums the generators see.
pub(crate) async fn postgres_enums(
    conn: &sqlx::PgPool,
) -> Result<Vec<DatabaseEnum>, CustomMigrationError> {
    let rows = sqlx::query(introspection::queries::ENUMS)
        .bind("public")
        .fetch_all(conn)
        .await
        .map_err(|e| CustomMigrationError::SendError(Box::new(e)))?;

    let mut by_name: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for row in rows {
        let name: String = row.get("name");
        let value: String = row.get("value");
        by_name.entry(name).or_default().push(value);
    }

    Ok(by_name
        .into_iter()
        .map(|(name, values)| DatabaseEnum {
            name,
            values,
            table: None,
            column: None,
        })
        .collect())
}
