//! Atomic multi-row inserts using the same New-input omission rules as create.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::inputs;
use crate::database::introspection::{Column, Table};

/// Emits one INSERT for a nonempty batch; empty input performs no database call.
/// Missing optional New fields use SQL DEFAULT, never invented Rust values.
/// Tables with no New fields are skipped rather than emitting invalid empty tuples.
/// The caller owns the executor and any transaction boundary.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names) -> Option<String> {
    let columns = inputs::new_columns(table);
    if columns.is_empty() {
        return None;
    }
    let column_sql = columns
        .iter()
        .map(|column| sql(&column.name))
        .collect::<Vec<_>>()
        .join(", ");
    let prefix = format!("INSERT INTO {} ({column_sql}) ", sql(&table.name));
    let values = columns
        .iter()
        .map(|column| value(table, column, names))
        .collect::<Vec<_>>()
        .join("\n");
    let function = names.function(table, "create_many");
    let create = names.create(table);
    let model = names.row(table);
    Some(format!(
        "/// Atomically inserts a batch; absent New values use database defaults.\n\
         /// Empty input makes no query; returned row order is not guaranteed.\n\
         /// One statement is used; database parameter limits apply without chunking.\n\
         /// Uses the supplied pool or transaction connection without committing it.\n\
         /// Example: `{function}(pool, &inputs).await?` returns generated rows.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, inputs: &[{create}]) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             if inputs.is_empty() {{ return Ok(Vec::new()); }}\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             query.push_values(inputs.iter(), |mut values, input| {{\n\
         {values}\n\
             }});\n\
             query.push(\" RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_all(executor).await\n\
         }}\n\n"
    ))
}

/// Emits one bound value or DEFAULT using the generated field's omission rule.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
fn value(table: &Table, column: &Column, names: &Names) -> String {
    let field = names.field(table, &column.name);
    if inputs::optional(column) {
        format!(
            "        if let Some(value) = input.{field}.as_ref() {{\n\
                 values.push_bind(value);\n\
             }} else {{ values.push(\"DEFAULT\"); }}"
        )
    } else {
        format!("        values.push_bind(&input.{field});")
    }
}
