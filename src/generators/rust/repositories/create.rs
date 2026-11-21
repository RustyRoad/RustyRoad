//! Insert emission; e.g. `create_users(pool, NewUsers {})` uses defaults for an auto-ID-only table.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::{fields, inputs};
use crate::database::introspection::Table;

/// Emits creation accepting a pool or a caller-owned connection without committing.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names) -> String {
    let function = names.function(table, "create");
    let model = names.row(table);
    let create = names.create(table);
    let columns = inputs::new_columns(table);
    let default_query = format!(
        "INSERT INTO {} DEFAULT VALUES RETURNING *",
        sql(&table.name)
    );
    if columns.is_empty() {
        return format!(
            "/// Creates one `{table}` row on the supplied executor without committing.\n\
             /// Example: `{function}(&mut *transaction, input).await?`.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {create} {{}}: {create}) -> Result<{model}, sqlx::Error> {{\n\
                 sqlx::query_as::<_, {model}>({default_query:?}).fetch_one(executor).await\n\
             }}",
            table = table.name,
        );
    }
    let has_values = if columns.iter().any(|column| !inputs::optional(column)) {
        "true".to_string()
    } else {
        inputs::present(table, &columns, names)
    };
    let (column_pushes, value_pushes) = fields::insert(table, &columns, names);

    format!(
        "/// Creates one `{table}` row on the supplied executor without committing.\n\
             /// Example: `{function}(&mut *transaction, input).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, input: {create}) -> Result<{model}, sqlx::Error> {{\n\
             if !({has_values}) {{\n\
                 return sqlx::query_as::<_, {model}>({default_query:?}).fetch_one(executor).await;\n\
             }}\n\n\
             let mut query = QueryBuilder::<Postgres>::new({insert_prefix:?});\n\
             {{\n\
                 let mut columns = query.separated(\", \" );\n\
         {column_pushes}\n\
             }}\n\
             query.push(\") VALUES (\");\n\
             {{\n\
                 let mut values = query.separated(\", \" );\n\
         {value_pushes}\n\
             }}\n\
             query.push(\") RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_one(executor).await\n\
         }}",
        table = table.name,
        insert_prefix = format!("INSERT INTO {} (", sql(&table.name)),
    )
}
