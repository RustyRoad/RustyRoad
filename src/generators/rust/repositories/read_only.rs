//! Read-only repositories for views and relations without a simple primary key.

use super::super::allocation::Names;
use super::super::naming::sql;
use crate::database::introspection::Table;

/// Emits a list operation using the existing allocated function and row names.
/// For example, `render(table, names)` emits `list_driver_performance_summary`
/// for that view; the function accepts a pool and returns generated row values.
/// This renderer never emits an insert, update, delete, or inferred identity.
pub(super) fn render(table: &Table, names: &Names) -> String {
    let function = names.function(table, "list");
    let model = names.row(table);
    let query = format!("SELECT * FROM {}", sql(&table.name));
    format!(
        "/// Reads every row from `{table_name}` without changing the relation.\n\
         /// Example: `{function}(pool).await?` returns generated `{model}` rows.\n\
         pub async fn {function}(pool: &PgPool) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             sqlx::query_as::<_, {model}>({query:?}).fetch_all(pool).await\n\
         }}\n\n",
        table_name = table.name,
    )
}
