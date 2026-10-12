//! `query_as` helpers: rows mapped into a `FromRow` type, as generated models use.

use crate::database::pool::shared::pool;
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use sqlx::{FromRow, Postgres};

/// Runs `query_as` and returns exactly one row.
pub async fn fetch_one<'q, T>(
    query: QueryAs<'q, Postgres, T, PgArguments>,
) -> Result<T, sqlx::Error>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    query.fetch_one(pool().await).await
}

/// Runs `query_as` and returns every row.
pub async fn fetch_all<'q, T>(
    query: QueryAs<'q, Postgres, T, PgArguments>,
) -> Result<Vec<T>, sqlx::Error>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    query.fetch_all(pool().await).await
}

/// Runs `query_as` and returns the row, if any.
pub async fn fetch_optional<'q, T>(
    query: QueryAs<'q, Postgres, T, PgArguments>,
) -> Result<Option<T>, sqlx::Error>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    query.fetch_optional(pool().await).await
}
