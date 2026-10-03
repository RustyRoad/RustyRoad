//! Per-table CRUD composition; `render(table, schema)` keeps read/delete SQL static.

use super::super::naming::{pascal, snake, sql};
use super::super::types;
use super::{create, update};
use crate::database::introspection::{Schema, Table};

/// Composes five functions; e.g. a `users` table emits `find_users(pool, id)`.
pub(super) fn render(table: &Table, schema: &Schema) -> String {
    let function = snake(&table.name);
    let model = pascal(&table.name);
    let key = table
        .column(&table.primary_key[0])
        .expect("introspected primary key should reference a column");
    let key_type = types::map(key, schema).rust;
    let table_sql = sql(&table.name);
    let key_sql = sql(&key.name);

    format!(
        "/// Returns every row from `{table_name}`.\n\
         pub async fn list_{function}(pool: &PgPool) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             sqlx::query_as::<_, {model}>({list_query:?}).fetch_all(pool).await\n\
         }}\n\n\
         /// Returns one `{table_name}` row by primary key.\n\
         pub async fn find_{function}(pool: &PgPool, id: {key_type}) -> Result<Option<{model}>, sqlx::Error> {{\n\
             sqlx::query_as::<_, {model}>({find_query:?})\n\
                 .bind(id)\n\
                 .fetch_optional(pool)\n\
                 .await\n\
         }}\n\n\
         {create}\n\
         {update}\n\
         /// Deletes one `{table_name}` row by primary key.\n\
         pub async fn delete_{function}(pool: &PgPool, id: {key_type}) -> Result<bool, sqlx::Error> {{\n\
             let result = sqlx::query({delete_query:?}).bind(id).execute(pool).await?;\n\
             Ok(result.rows_affected() > 0)\n\
         }}\n\n",
        table_name = table.name,
        list_query = format!("SELECT * FROM {table_sql}"),
        find_query = format!("SELECT * FROM {table_sql} WHERE {key_sql} = $1 LIMIT 1"),
        delete_query = format!("DELETE FROM {table_sql} WHERE {key_sql} = $1"),
        create = create::render(table),
        update = update::render(table, &key_type),
    )
}
