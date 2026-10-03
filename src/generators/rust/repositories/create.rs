//! Insert emission; e.g. `create_users(pool, NewUsers {})` uses defaults for an auto-ID-only table.

use super::super::naming::{pascal, snake, sql};
use super::{fields, inputs};
use crate::database::introspection::Table;

/// Emits create; e.g. `render(table)` destructures empty New inputs without a builder.
pub(super) fn render(table: &Table) -> String {
    let function = snake(&table.name);
    let model = pascal(&table.name);
    let columns = inputs::new_columns(table);
    let default_query = format!("INSERT INTO {} DEFAULT VALUES RETURNING *", sql(&table.name));
    if columns.is_empty() {
        return format!(
            "/// Creates one `{table}` row and returns it.\n\
             pub async fn create_{function}(pool: &PgPool, New{model} {{}}: New{model}) -> Result<{model}, sqlx::Error> {{\n\
                 sqlx::query_as::<_, {model}>({default_query:?}).fetch_one(pool).await\n\
             }}",
            table = table.name,
        );
    }
    let has_values = if columns.iter().any(|column| !inputs::optional(column)) {
        "true".to_string()
    } else {
        inputs::present(&columns)
    };
    let (column_pushes, value_pushes) = fields::insert(&columns);

    format!(
        "/// Creates one `{table}` row and returns it.\n\
         pub async fn create_{function}(pool: &PgPool, input: New{model}) -> Result<{model}, sqlx::Error> {{\n\
             if !({has_values}) {{\n\
                 return sqlx::query_as::<_, {model}>({default_query:?}).fetch_one(pool).await;\n\
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
             query.build_query_as::<{model}>().fetch_one(pool).await\n\
         }}",
        table = table.name,
        insert_prefix = format!("INSERT INTO {} (", sql(&table.name)),
    )
}