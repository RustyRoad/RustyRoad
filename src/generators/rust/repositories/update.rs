//! Patch emission; e.g. `update_users(pool, id, PatchUsers {})` finds a key-only row.

use super::super::naming::{pascal, snake, sql};
use super::{fields, inputs};
use crate::database::introspection::Table;

/// Emits update; e.g. `render(table, "i64")` keeps the ID before the Patch argument.
pub(super) fn render(table: &Table, key_type: &str) -> String {
    let function = snake(&table.name);
    let model = pascal(&table.name);
    let key = &table.primary_key[0];
    let mutable = inputs::patch_columns(table);
    if mutable.is_empty() {
        return format!(
            "/// Applies a partial update to one `{table}` row.\n\
             pub async fn update_{function}(pool: &PgPool, id: {key_type}, Patch{model} {{}}: Patch{model}) -> Result<Option<{model}>, sqlx::Error> {{\n\
                 find_{function}(pool, id).await\n\
             }}",
            table = table.name,
        );
    }
    let changed = inputs::present(&mutable);
    let assignments = mutable
        .iter()
        .map(|column| fields::assignment(column))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "/// Applies a partial update to one `{table}` row.\n\
         pub async fn update_{function}(pool: &PgPool, id: {key_type}, input: Patch{model}) -> Result<Option<{model}>, sqlx::Error> {{\n\
             if !({changed}) {{\n\
                 return find_{function}(pool, id).await;\n\
             }}\n\n\
             let mut query = QueryBuilder::<Postgres>::new({update_prefix:?});\n\
             {{\n\
                 let mut set = query.separated(\", \" );\n\
         {assignments}\n\
             }}\n\
             query.push({where_clause:?}).push_bind(id).push(\" RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_optional(pool).await\n\
         }}",
        table = table.name,
        update_prefix = format!("UPDATE {} SET ", sql(&table.name)),
        where_clause = format!(" WHERE {} = ", sql(key)),
    )
}