//! Per-table CRUD composition; `render(table, schema)` keeps read/delete SQL static.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::super::types;
use super::{create, update, update_touch};
use crate::database::introspection::{Schema, Table};

/// Composes CRUD and optional clock updates on a pool or caller-owned connection.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, schema: &Schema, names: &Names) -> String {
    let list = names.function(table, "list");
    let find = names.function(table, "find");
    let delete = names.function(table, "delete");
    let model = names.row(table);
    let key = table
        .column(&table.primary_key[0])
        .expect("introspected primary key should reference a column");
    let key_type = types::map_column(table, key, schema, names).rust;
    let table_sql = sql(&table.name);
    let key_sql = sql(&key.name);

    format!(
        "/// Returns every row from `{table_name}`.\n\
         pub async fn {list}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             sqlx::query_as::<_, {model}>({list_query:?}).fetch_all(executor).await\n\
         }}\n\n\
         /// Returns one `{table_name}` row by primary key.\n\
         pub async fn {find}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, id: {key_type}) -> Result<Option<{model}>, sqlx::Error> {{\n\
             sqlx::query_as::<_, {model}>({find_query:?})\n\
                 .bind(id)\n\
                 .fetch_optional(executor)\n\
                 .await\n\
         }}\n\n\
         {create}\n\
         {update}\n\
         {update_touch}\n\
         /// Deletes one `{table_name}` row by primary key.\n\
         pub async fn {delete}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, id: {key_type}) -> Result<bool, sqlx::Error> {{\n\
             let result = sqlx::query({delete_query:?}).bind(id).execute(executor).await?;\n\
             Ok(result.rows_affected() > 0)\n\
         }}\n\n",
        table_name = table.name,
        list_query = format!("SELECT * FROM {table_sql}"),
        find_query = format!("SELECT * FROM {table_sql} WHERE {key_sql} = $1 LIMIT 1"),
        delete_query = format!("DELETE FROM {table_sql} WHERE {key_sql} = $1"),
        create = create::render(table, names),
        update = update::render(table, &key_type, names),
        update_touch = update_touch::render(table, &key_type, names).unwrap_or_default(),
    )
}
