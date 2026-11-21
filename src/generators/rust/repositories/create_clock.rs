//! Opt-in database-time insertion without changing ordinary create semantics.

pub(in crate::generators::rust) mod fields;
pub(in crate::generators::rust) mod policy;

use super::super::allocation::Names;
use super::super::naming::sql;
use super::inputs;
use crate::database::introspection::Table;

/// Emit one-statement creation with explicitly selected omitted timestamp fields.
/// Supplied timestamps and explicit nested NULL values always retain precedence.
/// The generated operation accepts a pool or caller-owned connection without committing.
/// Invalid column requests fail before SQL execution; views receive no writes.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names) -> Option<String> {
    if table.view {
        return None;
    }
    let columns = inputs::new_columns(table);
    let clocks = columns
        .iter()
        .copied()
        .filter(|c| policy::eligible(table, c))
        .collect::<Vec<_>>();
    if clocks.is_empty() {
        return None;
    }
    let allowed = clocks
        .iter()
        .map(|c| format!("{:?}", c.name))
        .collect::<Vec<_>>()
        .join(" | ");
    let example = &clocks[0].name;
    let present = inputs::present(table, &columns, names);
    let required = columns.iter().any(|column| !inputs::optional(column));
    let has_values = if required {
        "true".into()
    } else {
        format!("!clocks.is_empty() || {present}")
    };
    let (column_pushes, value_pushes) = fields::render(table, &columns, names);
    let function = names.function(table, "create_clock");
    let model = names.row(table);
    let create = names.create(table);
    let prefix = format!("INSERT INTO {} (", sql(&table.name));
    let defaults = format!(
        "INSERT INTO {} DEFAULT VALUES RETURNING *",
        sql(&table.name)
    );
    Some(format!(
        "/// Creates one `{table_name}` row with selected omitted database timestamps.\n\
         /// Example: `{function}(&mut *transaction, input, &[{example:?}]).await?`.\n\
         /// Explicit values, including nested NULL, take precedence over clock selection.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, input: {create}, clocks: &[&str]) -> Result<{model}, sqlx::Error> {{\n\
             for clock in clocks {{\n\
                 if !matches!(*clock, {allowed}) {{\n\
                     return Err(sqlx::Error::Protocol(format!(\"Invalid database timestamp column: {{clock}}\")));\n\
                 }}\n\
             }}\n\
             if !({has_values}) {{\n\
                 return sqlx::query_as::<_, {model}>({defaults:?}).fetch_one(executor).await;\n\
             }}\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {{ let mut columns = query.separated(\", \");\n\
         {column_pushes}\n\
             }}\n\
             query.push(\") VALUES (\");\n\
             {{ let mut values = query.separated(\", \");\n\
         {value_pushes}\n\
             }}\n\
             query.push(\") RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_one(executor).await\n\
         }}\n\n",
        table_name = table.name,
    ))
}
