//! Attaching keys, constraints, indexes, and view-ness to an assembled schema.
//!
//! Kept apart from the reader because each of these is the same shape — run one catalog query,
//! group the rows, attach them — and inlining six of them made `read` a wall of plumbing.

use super::super::assemble::attach;
use super::super::model::Schema;
use super::super::queries;
use super::{fetch, group, rows};
use crate::database::migrations::CustomMigrationError;

/// Attaches everything that hangs off the bare column list.
pub(super) async fn attach_all(
    pool: &sqlx::PgPool,
    schema: &mut Schema,
    schema_name: &str,
) -> Result<(), CustomMigrationError> {
    let keys = fetch(pool, queries::PRIMARY_KEYS, schema_name).await?;
    attach::primary_keys(
        schema,
        keys.iter()
            .map(|row| {
                (
                    rows::text(row, "table_name"),
                    rows::text(row, "column_name"),
                )
            })
            .collect(),
    );

    // Marked before generation so a view's read-only nature is known when its model resolves.
    let views = fetch(pool, queries::VIEWS, schema_name).await?;
    attach::views(
        schema,
        views
            .iter()
            .map(|row| rows::text(row, "table_name"))
            .collect(),
    );

    let uniques = fetch(pool, queries::UNIQUES, schema_name).await?;
    attach::uniques(schema, group::constraints(uniques));

    let indexes = fetch(pool, queries::INDEXES, schema_name).await?;
    attach::indexes(schema, group::indexes(indexes));

    let foreign = fetch(pool, queries::FOREIGN_KEYS, schema_name).await?;
    attach::foreign_keys(schema, group::foreign_keys(foreign));

    Ok(())
}
