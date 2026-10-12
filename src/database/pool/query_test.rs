//! Compile-time check that the query shapes generated models emit fit the pool helpers.
//!
//! Nothing here runs: each function mirrors one renderer's output, and the test passes once
//! the module type-checks. A signature change that breaks generated code fails this build.

#![allow(dead_code)]

use super::{execute, fetch_all, fetch_one, fetch_optional, scalar_one};

#[derive(sqlx::FromRow)]
struct Row {
    id: i32,
}

/// `read.rs`: `all()` and `find(id)`.
async fn read(id: i32) -> Result<(Vec<Row>, Option<Row>), sqlx::Error> {
    let all = fetch_all(sqlx::query_as::<_, Row>("SELECT * FROM rows")).await?;
    let one = fetch_optional(sqlx::query_as::<_, Row>("SELECT * FROM rows WHERE id = $1").bind(id))
        .await?;
    Ok((all, one))
}

/// `read.rs` for a view: `all_options(...)` built with a `QueryBuilder`.
async fn filtered(id: Option<i32>) -> Result<Vec<Row>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM rows WHERE TRUE");
    if let Some(value) = id {
        query.push(" AND id = ").push_bind(value);
    }
    fetch_all(query.build_query_as::<Row>()).await
}

/// `create.rs` / `update.rs`: `INSERT ... RETURNING *` and `UPDATE ... RETURNING *`.
async fn write(id: i32) -> Result<Row, sqlx::Error> {
    fetch_one(sqlx::query_as::<_, Row>("UPDATE rows SET id = $1 RETURNING *").bind(id)).await
}

/// `delete.rs`: `delete(id)` counts affected rows.
async fn delete(id: i32) -> Result<bool, sqlx::Error> {
    let result = execute(sqlx::query("DELETE FROM rows WHERE id = $1").bind(id)).await?;
    Ok(result.rows_affected() > 0)
}

/// Scalars, which hand-written model code uses beside the generated methods.
async fn count() -> Result<i64, sqlx::Error> {
    scalar_one(sqlx::query_scalar("SELECT count(*) FROM rows")).await
}

#[test]
fn generated_query_shapes_type_check() {}
