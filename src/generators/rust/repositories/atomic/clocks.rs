//! Explicit database-clock upserts without application-side timestamp substitutes.
mod input;
mod insert;
mod many;
mod policy;
mod update;
use super::super::targets;
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{allocation::Names, naming::sql};

/// Declare one clock-aware insertion type for each eligible table.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn declaration(table: &Table, schema: &Schema, names: &Names) -> String {
    if targets::keys(table).is_empty() || !table.columns.iter().any(|c| policy::eligible(table, c))
    {
        return String::new();
    }
    input::render(table, schema, names)
}

/// Render a selective upsert with separate insertion and conflict clock lists.
/// Explicit values win; invalid clocks and missing mandatory timestamps fail
/// before SQL. No update fields means DO NOTHING and SQLx RowNotFound on conflict.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, key: &[String], names: &Names) -> String {
    if !table.columns.iter().any(|c| policy::eligible(table, c)) {
        return String::new();
    }
    let model = names.row(table);
    let input = names.clock_input(table);
    let patch = names.patch(table);
    let function = names.selector(table, "upsert_patch_clock", key);
    let target = key
        .iter()
        .map(|name| sql(name))
        .collect::<Vec<_>>()
        .join(", ");
    let conflict = format!(" ON CONFLICT ({target})");
    let validation = policy::validation(table, key, names);
    let insert = insert::render(table, names);
    let update = update::render(table, key, names);
    let mut output = format!(
        "/// Atomic selective upsert with explicit database clocks; supplied values take precedence.\n\
         /// Example: `{function}(pool, input, patch, &[], &[]).await`.\n\
         /// Unknown/key clock names or missing required timestamps fail before SQL execution.\n\
         pub async fn {function}(pool: &PgPool, input: {input}, patch: {patch}, insert_clocks: &[&str], update_clocks: &[&str]) -> Result<{model}, sqlx::Error> {{\n\
             {validation}\n{insert}\n\
             query.push({conflict:?});\n\
             {update}\n\
             query.push(\" RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_one(pool).await\n\
         }}\n\n"
    );
    output.push_str(&many::render(table, key, names));
    output
}
