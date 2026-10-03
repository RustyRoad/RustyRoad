//! Insert defaults plus an independent, selective conflict Patch.

use super::super::naming::{pascal, snake, sql};
use super::{insert, patch, targets};
use crate::database::introspection::Table;

/// Emits `upsert_sessions_patch_by_session_token(pool, input, patch)` atomically.
/// Only supplied Patch fields update conflicts; never copies unrelated New fields.
/// An empty conflict patch does nothing and fetch_one returns SQLx RowNotFound.
pub(super) fn render(table: &Table, key: &[String]) -> String {
    let function = format!("upsert_{}_patch_by_{}", snake(&table.name), targets::suffix(key));
    let model = pascal(&table.name);
    let patch::Patch { parameter, changed, assignments } = patch::render(table, "patch");
    let target = key.iter().map(|name| sql(name)).collect::<Vec<_>>().join(", ");
    let conflict = format!(" ON CONFLICT ({target})");
    let action = if assignments.is_empty() {
        "    query.push(\" DO NOTHING\");\n".to_string()
    } else {
        format!(
            "    if {changed} {{\n\
                     query.push(\" DO UPDATE SET \");\n\
                     {{\n\
                         let mut set = query.separated(\", \");\n\
             {assignments}\n\
                     }}\n\
                 }} else {{\n\
                     query.push(\" DO NOTHING\");\n\
                 }}\n"
        )
    };
    let insert = insert::render(table);
    format!(
        "/// Selective conflict patch; e.g. `{function}(pool, input, patch).await`.\n\
         /// An empty patch on conflict returns SQLx RowNotFound without a synthetic update.\n\
         pub async fn {function}(pool: &PgPool, input: New{model}, {parameter}) -> Result<{model}, sqlx::Error> {{\n\
         {insert}\
             query.push({conflict:?});\n\
         {action}\
             query.push(\" RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_one(pool).await\n\
         }}\n\n"
    )
}
