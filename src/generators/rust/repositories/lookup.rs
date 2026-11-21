//! Bound equality lookups using generated rows rather than application projections.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::selector;
use crate::database::introspection::{Column, Schema, Table};

/// Emits optional reads; e.g. key `page_id` yields `find_heatmaps_by_page_id(pool, value_1)`.
/// A latest lookup, e.g. by URL, explicitly retains PostgreSQL DESC NULLS FIRST.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    table: &Table,
    schema: &Schema,
    key: &[String],
    clock: Option<&Column>,
    names: &Names,
) -> String {
    let model = names.row(table);
    let selector::Selector {
        parameters,
        predicate,
        binds,
        arguments,
    } = selector::render(table, schema, key, names);
    let (verb, order) = match clock {
        Some(column) => (
            "latest",
            format!(" ORDER BY {} DESC NULLS FIRST", sql(&column.name)),
        ),
        None => ("find", String::new()),
    };
    let query = format!(
        "SELECT * FROM {} WHERE {predicate}{order} LIMIT 1",
        sql(&table.name)
    );
    let function = names.selector(table, verb, key);
    format!(
        "/// Optional row lookup; e.g. `{function}(pool, {arguments}).await`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {parameters}) -> Result<Option<{model}>, sqlx::Error> {{\n\
             sqlx::query_as::<_, {model}>({query:?}){binds}.fetch_optional(executor).await\n\
         }}\n\n"
    )
}
