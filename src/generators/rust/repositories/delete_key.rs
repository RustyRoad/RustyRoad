//! One-statement deletion through verified unique selectors.

use super::super::naming::{snake, sql};
use super::{selector, targets};
use crate::database::introspection::{Schema, Table};

/// Emits a counted delete; e.g. `delete_sessions_by_session_token(pool, value_1)`.
/// No preliminary read is needed: SQLx returns the affected-row count atomically.
pub(super) fn render(table: &Table, schema: &Schema, key: &[String]) -> String {
    let function = format!("delete_{}_by_{}", snake(&table.name), targets::suffix(key));
    let selector::Selector { parameters, predicate, binds, arguments } = selector::render(table, schema, key);
    let query = format!("DELETE FROM {} WHERE {predicate}", sql(&table.name));
    format!(
        "/// Deletes matching rows; e.g. `{function}(pool, {arguments}).await` returns their count.\n\
         pub async fn {function}(pool: &PgPool, {parameters}) -> Result<u64, sqlx::Error> {{\n\
             let result = sqlx::query({query:?}){binds}.execute(pool).await?;\n\
             Ok(result.rows_affected())\n\
         }}\n\n"
    )
}
