//! Patch emission; e.g. `update_users(pool, id, PatchUsers {})` finds a key-only row.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::{fields, inputs};
use crate::database::introspection::Table;

/// Emits a partial update on the supplied pool or transaction connection.
/// Empty patches read on that same executor without opening another connection.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, key_type: &str, names: &Names) -> String {
    let function = names.function(table, "update");
    let find = names.function(table, "find");
    let model = names.row(table);
    let patch = names.patch(table);
    let key = &table.primary_key[0];
    let mutable = inputs::patch_columns(table);
    if mutable.is_empty() {
        return format!(
            "/// Applies a partial update on the supplied executor; never commits.\n\
             /// Example: `{function}(&mut *transaction, id, patch).await?`.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, id: {key_type}, {patch} {{}}: {patch}) -> Result<Option<{model}>, sqlx::Error> {{\n\
                 {find}(executor, id).await\n\
             }}",
        );
    }
    let changed = inputs::present(table, &mutable, names);
    let assignments = mutable
        .iter()
        .map(|column| fields::assignment(table, column, names))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "/// Applies a partial update on the supplied executor; never commits.\n\
             /// Example: `{function}(&mut *transaction, id, patch).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, id: {key_type}, input: {patch}) -> Result<Option<{model}>, sqlx::Error> {{\n\
             if !({changed}) {{\n\
                 return {find}(executor, id).await;\n\
             }}\n\n\
             let mut query = QueryBuilder::<Postgres>::new({update_prefix:?});\n\
             {{\n\
                 let mut set = query.separated(\", \" );\n\
         {assignments}\n\
             }}\n\
             query.push({where_clause:?}).push_bind(id).push(\" RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_optional(executor).await\n\
         }}",
        update_prefix = format!("UPDATE {} SET ", sql(&table.name)),
        where_clause = format!(" WHERE {} = ", sql(key)),
    )
}
