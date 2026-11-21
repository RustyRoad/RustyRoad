//! Native atomic conflict policies over schema-derived payloads.

use super::super::allocation::Names;
use super::super::naming::{snake, sql};
use super::{insert, policy};
use crate::database::introspection::{Column, Table};

/// Emits one statement; e.g. `upsert_heatmaps_merge_data_by_page_id(pool, input)`.
/// `merge == None` selects snapshot replacement, not JSON concatenation.
pub(super) fn render(
    table: &Table,
    key: &[String],
    merge: Option<&Column>,
    names: &Names,
) -> String {
    let payload = match merge {
        Some(column) => vec![column],
        None => policy::payload(table, key),
    };
    let mut assignments = payload
        .iter()
        .map(|column| {
            let name = sql(&column.name);
            if merge.is_some() {
                format!("{name} = {}.{name} || EXCLUDED.{name}", sql("existing"))
            } else {
                format!("{name} = EXCLUDED.{name}")
            }
        })
        .collect::<Vec<_>>();
    if let Some(clock) = policy::clock(table).filter(|clock| !key.contains(&clock.name)) {
        assignments.push(format!("{} = CURRENT_TIMESTAMP", sql(&clock.name)));
    }
    if assignments.is_empty() {
        return String::new();
    }
    let mode = merge
        .map(|column| format!("merge_{}", snake(&column.name)))
        .unwrap_or_else(|| "snapshot".to_string());
    let function = match merge {
        Some(column) => names.merge(table, key, &column.name),
        None => names.selector(table, "upsert_snapshot", key),
    };
    let model = names.row(table);
    let create = names.create(table);
    let target = key
        .iter()
        .map(|name| sql(name))
        .collect::<Vec<_>>()
        .join(", ");
    let conflict = format!(
        " ON CONFLICT ({target}) DO UPDATE SET {} RETURNING *",
        assignments.join(", ")
    );
    let insert = insert::render(table, names);
    format!(
        "/// Atomic {mode}; e.g. `{function}(pool, input).await` returns the stored row.\n\
         pub async fn {function}(pool: &PgPool, input: {create}) -> Result<{model}, sqlx::Error> {{\n\
         {insert}\
             query.push({conflict:?});\n\
             query.build_query_as::<{model}>().fetch_one(pool).await\n\
         }}\n\n"
    )
}
