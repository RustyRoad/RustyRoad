//! `query_scalar` helpers: one decoded column per row.

use crate::database::pool::shared::pool;
use sqlx::postgres::PgArguments;
use sqlx::query::QueryScalar;
use sqlx::{Decode, Postgres, Type};

/// Runs `query_scalar` and returns exactly one value.
pub async fn scalar_one<'q, T>(
    query: QueryScalar<'q, Postgres, T, PgArguments>,
) -> Result<T, sqlx::Error>
where
    T: Send + Unpin + Type<Postgres> + for<'r> Decode<'r, Postgres>,
{
    query.fetch_one(pool().await).await
}

/// Runs `query_scalar` and returns the value, if any.
pub async fn scalar_optional<'q, T>(
    query: QueryScalar<'q, Postgres, T, PgArguments>,
) -> Result<Option<T>, sqlx::Error>
where
    T: Send + Unpin + Type<Postgres> + for<'r> Decode<'r, Postgres>,
{
    query.fetch_optional(pool().await).await
}

/// Runs `query_scalar` and returns every value.
pub async fn scalar_all<'q, T>(
    query: QueryScalar<'q, Postgres, T, PgArguments>,
) -> Result<Vec<T>, sqlx::Error>
where
    T: Send + Unpin + Type<Postgres> + for<'r> Decode<'r, Postgres>,
{
    query.fetch_all(pool().await).await
}
