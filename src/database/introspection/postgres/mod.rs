//! Executing catalog queries against Postgres.

mod group;
mod rows;

use super::assemble::{self, attach};
use super::model::Schema;
use super::queries;
use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;
use sqlx::postgres::PgRow;

/// Reads the full schema, including keys, constraints, and indexes.
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

    let keys = fetch(pool, queries::PRIMARY_KEYS, schema_name).await?;
    attach::primary_keys(
        &mut schema,
        keys.iter()
            .map(|row| (rows::text(row, "table_name"), rows::text(row, "column_name")))
            .collect(),
    );

    let uniques = fetch(pool, queries::UNIQUES, schema_name).await?;
    attach::uniques(&mut schema, group::constraints(uniques));

    let indexes = fetch(pool, queries::INDEXES, schema_name).await?;
    attach::indexes(&mut schema, group::indexes(indexes));

    let foreign = fetch(pool, queries::FOREIGN_KEYS, schema_name).await?;
    attach::foreign_keys(&mut schema, group::foreign_keys(foreign));

    Ok(schema)
}

/// Runs one catalog query bound to a schema name.
async fn fetch(
    pool: &sqlx::PgPool,
    sql: &str,
    schema_name: &str,
) -> Result<Vec<PgRow>, CustomMigrationError> {
    sqlx::query(sql)
        .bind(schema_name)
        .fetch_all(pool)
        .await
        .map_err(CustomMigrationError::SqlxError)
}
