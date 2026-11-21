//! Database transaction time for caller-owned multi-statement workflows.

/// Fixed generated helper identity reserved against table and configured functions.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) const FUNCTION: &str = "database_transaction_time";

/// Emits a typed PostgreSQL clock read without acquiring or committing a connection.
/// Only a shared transaction guarantees identical time across related statements.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render() -> String {
    format!(
        "/// Reads PostgreSQL transaction time on the supplied executor.\n\
         /// Use the same transaction connection for this read and related writes.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {{\n\
         /// let mut transaction = pool.begin().await?;\n\
         /// let time = db::repositories::{FUNCTION}(&mut *transaction).await?;\n\
         /// println!(\"{{time}}\");\n\
         /// transaction.rollback().await?;\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {FUNCTION}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>) -> Result<chrono::DateTime<chrono::Utc>, sqlx::Error> {{\n\
             sqlx::query_scalar(\"SELECT CURRENT_TIMESTAMP\").fetch_one(executor).await\n\
         }}\n\n"
    )
}
