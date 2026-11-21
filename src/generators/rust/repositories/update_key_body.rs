//! Bound UPDATE bodies shared by ordinary and timestamp-touch selectors.

use super::super::naming::sql;
use super::{patch::Patch, where_key};
use crate::database::introspection::Table;

/// Render a keyed update without changing ordinary empty-patch behavior.
/// The touch path always has a clock assignment and never substitutes a read.
/// Generated operations retain the supplied pool or transaction connection.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(
    table: &Table,
    key: &[String],
    model: &str,
    patch: &Patch,
    lookup: &str,
    touch: bool,
) -> (String, bool) {
    if patch.assignments.is_empty() {
        return (lookup.to_owned(), false);
    }
    let prefix = format!("UPDATE {} SET ", sql(&table.name));
    let filter = where_key::render(key);
    let guard = if touch {
        String::new()
    } else {
        format!("    if !({}) {{ return {lookup}; }}\n", patch.changed)
    };
    let body = format!(
        "{guard}\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {{\n\
                 let mut set = query.separated(\", \");\n\
         {assignments}\n\
             }}\n\
         {filter}\
             query.push(\" RETURNING *\");\n\
             query.build_query_as::<{model}>().fetch_one(executor).await",
        assignments = patch.assignments,
    );
    (body, true)
}
