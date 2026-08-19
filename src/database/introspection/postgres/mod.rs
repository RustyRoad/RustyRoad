//! Executing catalog queries against Postgres.

mod constraints;
mod group;
mod rows;

use super::assemble;
use super::model::Schema;
use super::queries;
use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;
use sqlx::postgres::PgRow;

/// Reads the full schema, including keys, constraints, indexes, and view-ness.
pub async fn read(
    connection: &DatabaseConnection,
    schema_name: &str,
) -> Result<Schema, CustomMigrationError> {
    let DatabaseConnection::Pg(pool) = connection else {
        return Ok(Schema::default());
    };
    let pool = pool.as_ref();

    let columns = fetch(pool, queries::COLUMNS, schema_name).await?;
    let enums = fetch(pool, queries::ENUMS, schema_name).await?;
    let mut schema = Schema {
        tables: assemble::tables(columns.iter().map(rows::column).collect()),
        enums: group::enums(enums),
    };

    constraints::attach_all(pool, &mut schema, schema_name).await?;

    Ok(schema)
}

/// Runs one catalog query bound to a schema name.
pub(super) async fn fetch(
    pool: &sqlx::PgPool,
    sql: &str,
    schema_name: &str,
) -> Result<Vec<PgRow>, CustomMigrationError> {
    sqlx::query(sqlx::AssertSqlSafe(sql.to_owned()))
        .bind(schema_name)
        .fetch_all(pool)
        .await
        .map_err(CustomMigrationError::SqlxError)
}
