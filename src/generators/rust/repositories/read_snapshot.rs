//! Read-only transaction snapshots for composed generated repository reads.

/// Shared generated identity reserved against table and configured operations.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) const FUNCTION: &str = "begin_read_snapshot";

/// Emits a pool-owned repeatable-read transaction without application SQL strings.
/// Callers pass the same transaction connection to every related generated read.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render() -> String {
    format!(
        "/// Starts a read-only, repeatable-read transaction for consistent generated reads.\n\
         /// The snapshot is established by the first query; callers must finish the transaction.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {{\n\
         /// let mut transaction = db::repositories::{FUNCTION}(pool).await?;\n\
         /// let timestamp = db::repositories::database_transaction_time(&mut *transaction).await?;\n\
         /// println!(\"{{timestamp}}\");\n\
         /// transaction.rollback().await?;\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {FUNCTION}(pool: &sqlx::PgPool) -> Result<sqlx::Transaction<'static, sqlx::Postgres>, sqlx::Error> {{\n\
             pool.begin_with(\"BEGIN TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY\").await\n\
         }}\n\n"
    )
}
