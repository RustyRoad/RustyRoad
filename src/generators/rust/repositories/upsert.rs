//! Native atomic conflict policies over schema-derived payloads.

use super::super::naming::{pascal, snake, sql};
use super::{insert, policy, targets};
use crate::database::introspection::{Column, Table};

/// Emits one statement; e.g. `upsert_heatmaps_merge_data_by_page_id(pool, input)`.
/// `merge == None` selects snapshot replacement, not JSON concatenation.
pub(super) fn render(table: &Table, key: &[String], merge: Option<&Column>) -> String {
    let payload = match merge {
        Some(column) => vec![column],
        None => policy::payload(table, key),
    };
    let mut assignments = payload.iter().map(|column| {
        let name = sql(&column.name);
        if merge.is_some() {
            format!("{name} = {}.{name} || EXCLUDED.{name}", sql("existing"))
        } else {
            format!("{name} = EXCLUDED.{name}")
        }
    }).collect::<Vec<_>>();
    if let Some(clock) = policy::clock(table).filter(|clock| !key.contains(&clock.name)) {
        assignments.push(format!("{} = CURRENT_TIMESTAMP", sql(&clock.name)));
    }
    if assignments.is_empty() {
        return String::new();
    }
    let mode = merge.map(|column| format!("merge_{}", snake(&column.name)))
        .unwrap_or_else(|| "snapshot".to_string());
    let function = format!("upsert_{}_{}_by_{}", snake(&table.name), mode, targets::suffix(key));
    let model = pascal(&table.name);
    let target = key.iter().map(|name| sql(name)).collect::<Vec<_>>().join(", ");
    let conflict = format!(" ON CONFLICT ({target}) DO UPDATE SET {} RETURNING *", assignments.join(", "));
    let insert = insert::render(table);
    format!(
        "/// Atomic {mode}; e.g. `{function}(pool, input).await` returns the stored row.\n\
         pub async fn {function}(pool: &PgPool, input: New{model}) -> Result<{model}, sqlx::Error> {{\n\
         {insert}\
             query.push({conflict:?});\n\
             query.build_query_as::<{model}>().fetch_one(pool).await\n\
         }}\n\n"
    )
}
