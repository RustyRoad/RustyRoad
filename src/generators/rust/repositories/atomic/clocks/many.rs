use super::{policy, update};
mod values;
use crate::database::introspection::Table;
use crate::generators::rust::repositories::inputs;
use crate::generators::rust::{allocation::Names, naming::sql};

/// Emits an atomic batch upsert with independent insert/update clock selection.
/// Uses the same clock-aware input and conflict-patch policy as the single-row operation.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, key: &[String], names: &Names) -> String {
    let columns = inputs::new_columns(table);
    let selected = columns
        .iter()
        .map(|column| sql(&column.name))
        .collect::<Vec<_>>()
        .join(", ");
    let prefix = format!(
        "INSERT INTO {} AS {} ({selected}) ",
        sql(&table.name),
        sql("existing")
    );
    let conflict = format!(
        " ON CONFLICT ({})",
        key.iter()
            .map(|name| sql(name))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let values = columns
        .iter()
        .map(|column| values::render(table, column, names))
        .collect::<Vec<_>>()
        .join("\n");
    let validation = policy::validation(table, key, names);
    let update = update::render(table, key, names);
    let input = names.clock_input(table);
    let patch = names.patch(table);
    let function = names.selector(table, "upsert_many_patch_clock", key);
    format!(
        "/// Atomically upserts a batch with explicit database clocks and one shared patch.\n\
         /// Empty input returns zero without querying; no chunking or deduplication occurs.\n\
         /// Explicit values (including NULL) win over clocks; parameter/cardinality errors propagate.\n\
         /// Returns affected rows without decoding returned rows or committing the executor.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, inputs: &[{input}], patch: {patch}) -> Result<(), sqlx::Error> {{\n\
         /// let affected = {function}(pool, inputs, patch, &[], &[]).await?;\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, inputs: &[{input}], patch: {patch}, insert_clocks: &[&str], update_clocks: &[&str]) -> Result<u64, sqlx::Error> {{\n\
             if inputs.is_empty() {{ return Ok(0); }}\n\
             for input in inputs {{ {validation} }}\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             query.push_values(inputs.iter(), |mut values, input| {{\n{values}\n }});\n\
             query.push({conflict:?});\n\
             {update}\n\
             query.build().execute(executor).await.map(|result| result.rows_affected())\n\
         }}\n\n"
    )
}
