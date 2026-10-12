//! Raw `query` helpers: statements and undecoded `PgRow`s.

use crate::database::pool::shared::pool;
use sqlx::postgres::{PgArguments, PgQueryResult, PgRow};
use sqlx::query::Query;
use sqlx::Postgres;

/// A raw statement bound with Postgres arguments.
type Raw<'q> = Query<'q, Postgres, PgArguments>;

/// Runs a statement that returns no rows, such as a `DELETE`.
pub async fn execute(query: Raw<'_>) -> Result<PgQueryResult, sqlx::Error> {
    query.execute(pool().await).await
}

/// Runs a raw `query` and returns exactly one row.
pub async fn query_one(query: Raw<'_>) -> Result<PgRow, sqlx::Error> {
    query.fetch_one(pool().await).await
}

/// Runs a raw `query` and returns every row.
pub async fn query_all(query: Raw<'_>) -> Result<Vec<PgRow>, sqlx::Error> {
    query.fetch_all(pool().await).await
}

/// Runs a raw `query` and returns the row, if any.
pub async fn query_optional(query: Raw<'_>) -> Result<Option<PgRow>, sqlx::Error> {
    query.fetch_optional(pool().await).await
}
