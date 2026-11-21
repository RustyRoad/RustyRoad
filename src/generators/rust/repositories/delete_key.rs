//! One-statement deletion through schema-derived equality selectors.

use super::super::allocation::Names;
use super::super::naming::sql;
use super::selector;
use crate::database::introspection::{Schema, Table};

/// Emits a counted delete; e.g. `delete_sessions_by_session_token(pool, value_1)`.
/// No preliminary read is needed: SQLx returns the affected-row count atomically.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, schema: &Schema, key: &[String], names: &Names) -> String {
    let function = names.selector(table, "delete", key);
    let selector::Selector {
        parameters,
        predicate,
        binds,
        arguments,
    } = selector::render(table, schema, key, names);
    let query = format!("DELETE FROM {} WHERE {predicate}", sql(&table.name));
    format!(
        "/// Deletes matching rows; e.g. `{function}(pool, {arguments}).await` returns their count.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {parameters}) -> Result<u64, sqlx::Error> {{\n\
             let result = sqlx::query({query:?}){binds}.execute(executor).await?;\n\
             Ok(result.rows_affected())\n\
         }}\n\n"
    )
}
