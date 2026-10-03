//! Bound equality lookups using generated rows rather than application projections.

use super::super::naming::{pascal, snake, sql};
use super::{selector, targets};
use crate::database::introspection::{Column, Schema, Table};

/// Emits optional reads; e.g. key `page_id` yields `find_heatmaps_by_page_id(pool, value_1)`.
/// A latest lookup, e.g. by URL, explicitly retains PostgreSQL DESC NULLS FIRST.
pub(super) fn render(table: &Table, schema: &Schema, key: &[String], clock: Option<&Column>) -> String {
    let model = pascal(&table.name);
    let function = snake(&table.name);
    let suffix = targets::suffix(key);
    let selector::Selector { parameters, predicate, binds, arguments } = selector::render(table, schema, key);
    let (verb, order) = match clock {
        Some(column) => ("latest", format!(" ORDER BY {} DESC NULLS FIRST", sql(&column.name))),
        None => ("find", String::new()),
    };
    let query = format!("SELECT * FROM {} WHERE {predicate}{order} LIMIT 1", sql(&table.name));
    format!(
        "/// Optional row lookup; e.g. `{verb}_{function}_by_{suffix}(pool, {arguments}).await`.\n\
         pub async fn {verb}_{function}_by_{suffix}(pool: &PgPool, {parameters}) -> Result<Option<{model}>, sqlx::Error> {{\n\
             sqlx::query_as::<_, {model}>({query:?}){binds}.fetch_optional(pool).await\n\
         }}\n\n"
    )
}
