//! Explicit whole-table deletion for caller-owned replacement transactions.
use super::super::{allocation::Names, naming::sql};
use crate::database::introspection::Table;

/// Emits a distinct operation rather than allowing empty filtered deletion.
/// The repository composer invokes this only for writable tables.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, names: &Names) -> String {
    let function = names.function(table, "delete_all");
    let query = format!("DELETE FROM {}", sql(&table.name));
    format!(
        "/// Deletes every row, returning the count; database errors propagate.\n\
         /// This explicit operation has no predicate and never commits the executor.\n\
         /// Use the same transaction connection for a following replacement insert.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {{\n\
         /// let mut transaction = pool.begin().await?;\n\
         /// let deleted = {function}(&mut *transaction).await?;\n\
         /// transaction.rollback().await?;\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>) -> Result<u64, sqlx::Error> {{\n\
             Ok(sqlx::query({query:?}).execute(executor).await?.rows_affected())\n\
         }}\n\n"
    )
}
