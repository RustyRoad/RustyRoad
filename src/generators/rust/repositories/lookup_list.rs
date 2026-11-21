//! Generated collection reads for schema-backed foreign-key equality selectors.
use super::super::{allocation::Names, naming::sql};
use super::selector;
use crate::database::introspection::{Schema, Table};

/// Emit a bound collection read; no arbitrary LIMIT or implicit order is introduced.
/// A caller needing order must apply its existing domain ordering explicitly.
/// Generated operations retain the supplied pool or transaction connection.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, schema: &Schema, key: &[String], names: &Names) -> String {
    let model = names.row(table);
    let function = names.selector(table, "list", key);
    let selector::Selector {
        parameters,
        predicate,
        binds,
        arguments,
    } = selector::render(table, schema, key, names);
    let query = format!("SELECT * FROM {} WHERE {predicate}", sql(&table.name));
    format!(
        "/// Lists matching rows; e.g. `{function}(pool, {arguments}).await`; order is unspecified.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {parameters}) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             sqlx::query_as::<_, {model}>({query:?}){binds}.fetch_all(executor).await\n\
         }}\n\n"
    )
}
