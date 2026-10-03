//! Batched backfill of existing rows.
//!
//! Existing rows have no value in a newly created shadow column. The trigger only
//! fires on write, so rows that are never written would stay empty. The backfill
//! walks the table in batches, touching each row so the trigger computes its value.
//!
//! Batching matters: a single `UPDATE` over a large table holds row locks for its
//! whole duration, which is the outage this design exists to avoid.

mod batch;
mod clauses;
mod keys;

pub use batch::batch as batch_sql;

use crate::database::migrations::CustomMigrationError;
use crate::database::DatabaseConnection;
use sqlx::Row as _;
use std::io;

/// Rows touched per batch.
pub const DEFAULT_BATCH_SIZE: u32 = 1000;

/// Maximum batches before giving up, guarding against a non-terminating loop.
const MAX_BATCHES: u32 = 100_000;

/// Backfills `table`, returning the number of batches executed.
pub async fn run(
    connection: &DatabaseConnection,
    table: &str,
    batch_size: u32,
) -> Result<u32, CustomMigrationError> {
    let DatabaseConnection::Pg(pool) = connection else {
        // Only Postgres runs the versioned lifecycle.
        return Ok(0);
    };

    let primary_key = keys::primary_key(connection, table).await?;
    if primary_key.is_empty() {
        return Err(CustomMigrationError::IoError(io::Error::other(format!(
            "Table '{table}' has no primary key, so it cannot be backfilled in batches. \
             Add a primary key, or use a raw SQL operation."
        ))));
    }

    let mut last: Option<Vec<String>> = None;
    for executed in 0..MAX_BATCHES {
        let sql = batch::batch(table, &primary_key, batch_size, last.as_deref());
        let row = sqlx::query(sqlx::AssertSqlSafe(sql))
            .fetch_optional(pool.as_ref())
            .await
            .map_err(CustomMigrationError::SqlxError)?;

        // No rows returned means every marked row has been processed.
        let Some(row) = row else {
            return Ok(executed);
        };

        last = Some(
            (0..primary_key.len())
                .map(|index| row.try_get::<String, _>(index).unwrap_or_default())
                .collect(),
        );
    }

    Err(CustomMigrationError::IoError(io::Error::other(format!(
        "Backfill of '{table}' did not finish within {MAX_BATCHES} batches."
    ))))
}
