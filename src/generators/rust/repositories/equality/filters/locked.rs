//! Explicit row locks stay on caller-owned transaction connections.
use super::{sql, Names, Table};

/// Emits a bounded locking read without committing or acquiring another connection.
/// Views are excluded and empty predicates fail before database access.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names, bindings: &str) -> String {
    if table.view {
        return String::new();
    }
    let model = names.row(table);
    let filter = names.filter(table);
    let function = names.function(table, "filter_first_for_update");
    let present = super::keys(table)
        .iter()
        .map(|key| format!("filter.{}.is_some()", names.field(table, &key[0])))
        .collect::<Vec<_>>()
        .join(" || ");
    let prefix = format!("SELECT * FROM {} WHERE TRUE", sql(&table.name));
    format!(
        "/// Locks one matching row on the supplied transaction connection.\n\
         /// No implicit ordering, connection acquisition or commit; empty predicates fail.\n\
         /// Example: `{function}(&mut *transaction, filter).await?`.\n\
         pub async fn {function}(connection: &mut sqlx::PgConnection, filter: {filter}) -> Result<Option<{model}>, sqlx::Error> {{\n\
             if !({present}) {{\n\
                 return Err(sqlx::Error::Protocol(\"Locking reads require a predicate\".into()));\n\
             }}\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             query.push(\" LIMIT 1 FOR UPDATE\");\n\
             query.build_query_as::<{model}>().fetch_optional(connection).await\n\
         }}\n\n"
    )
}
