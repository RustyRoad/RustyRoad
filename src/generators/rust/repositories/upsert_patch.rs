//! Insert defaults plus an independent, selective conflict Patch.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::{insert, patch};
use crate::database::introspection::{Column, Table};

/// Emits `upsert_sessions_patch_by_session_token(pool, input, patch)` atomically.
/// Only supplied Patch fields update conflicts; never copies unrelated New fields.
/// An empty conflict patch does nothing and fetch_one returns SQLx RowNotFound.
/// A clock opt-in touches transaction time only if its Patch field is omitted.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(
    table: &Table,
    key: &[String],
    names: &Names,
    clock: Option<&Column>,
) -> String {
    let mode = if clock.is_some() {
        "upsert_patch_touch"
    } else {
        "upsert_patch"
    };
    let function = names.selector(table, mode, key);
    let model = names.row(table);
    let create = names.create(table);
    let patch::Patch {
        parameter,
        changed,
        mut assignments,
    } = patch::render(table, "patch", names);
    if let Some(clock) = clock {
        let field = names.field(table, &clock.name);
        let touch = format!("{} = CURRENT_TIMESTAMP", sql(&clock.name));
        assignments.push_str(&format!(
            "\n        if patch.{field}.is_none() {{ set.push({touch:?}); }}"
        ));
    }
    let target = key
        .iter()
        .map(|name| sql(name))
        .collect::<Vec<_>>()
        .join(", ");
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
    let insert = insert::render(table, names);
    let clock_doc = if clock.is_some() {
        "/// Omitted update timestamps use transaction time; explicit Patch values are preserved.\n"
    } else {
        ""
    };
    format!(
        "/// Selective conflict patch; e.g. `{function}(pool, input, patch).await`.\n\
         /// An empty patch on conflict returns SQLx RowNotFound without a synthetic update.\n\
         {clock_doc}\
         pub async fn {function}(pool: &PgPool, input: {create}, {parameter}) -> Result<{model}, sqlx::Error> {{\n\
         {insert}\
             query.push({conflict:?});\n\
         {action}\
             query.push(\" RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_one(pool).await\n\
         }}\n\n"
    )
}
