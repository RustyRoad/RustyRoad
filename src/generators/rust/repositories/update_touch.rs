//! Explicit primary-key updates with a database-owned modification timestamp.
mod preserving;

use super::super::allocation::Names;
use super::super::naming::sql;
use super::{fields, inputs, policy};
use crate::database::introspection::Table;

/// Emits an opt-in update only when `updated_at` is an eligible timestamp.
/// Explicit patch values, including nullable values, retain their normal meaning.
/// An omitted timestamp uses transaction time, even for an otherwise empty patch.
/// Accepts the caller's connection without opening or committing a transaction.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, key_type: &str, names: &Names) -> Option<String> {
    let clock = policy::clock(table)?;
    let model = names.row(table);
    let patch = names.patch(table);
    let field = names.field(table, &clock.name);
    let clock_sql = format!("{} = CURRENT_TIMESTAMP", sql(&clock.name));
    let assignments = inputs::patch_columns(table)
        .iter()
        .map(|column| fields::assignment(table, column, names))
        .collect::<Vec<_>>()
        .join("\n");
    let mut output = String::new();
    for (verb, result, returning, finish, description) in [
        (
            "update_touch",
            format!("Option<{model}>"),
            "query.push(\" RETURNING *\");",
            format!("query.build_query_as::<{model}>().fetch_optional(executor).await"),
            "Returns the updated row, or None when the ID is absent.",
        ),
        (
            "update_touch_affected",
            "u64".to_owned(),
            "",
            "Ok(query.build().execute(executor).await?.rows_affected())".to_owned(),
            "Returns the affected count without decoding a row; a missing ID returns zero.",
        ),
    ] {
        let function = names.function(table, verb);
        output.push_str(&format!(
            "/// Updates one `{table_name}` row and touches an omitted modification time.\n\
             /// {description} Database errors propagate; no transaction is committed.\n\
             /// Example: `{function}(&mut *transaction, id, patch).await?` preserves explicit timestamps.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, id: {key_type}, input: {patch}) -> Result<{result}, sqlx::Error> {{\n\
                 let mut query = QueryBuilder::<Postgres>::new({update_prefix:?});\n\
                 {{\n\
                     let mut set = query.separated(\", \");\n\
             {assignments}\n\
                     if input.{field}.is_none() {{ set.push({clock_sql:?}); }}\n\
                 }}\n\
                 query.push({where_clause:?}).push_bind(id);\n\
                 {returning}\n\
                 {finish}\n\
             }}\n\n",
            table_name = table.name,
            update_prefix = format!("UPDATE {} SET ", sql(&table.name)),
            where_clause = format!(" WHERE {} = ", sql(&table.primary_key[0])),
        ));
    }
    output.push_str(&preserving::render(table, key_type, names));
    Some(output)
}
