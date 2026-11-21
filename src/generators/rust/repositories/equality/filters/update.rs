//! Single-statement selective updates with typed OR-of-AND predicates.
use super::{sql, Names, Table};
use crate::database::introspection::Column;
use crate::generators::rust::repositories::patch;

/// Emits bound bulk updates, rejecting absent or unconstrained predicate groups.
/// Views have no mutation operation. An empty patch affects zero rows without SQL.
/// Generated operations use caller-owned executors and never commit transactions.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names, columns: &[&Column], bindings: &str) -> String {
    if table.view || columns.is_empty() {
        return String::new();
    }
    let function = names.function(table, "filter_any_update");
    let filter = names.filter(table);
    let patch = patch::render(table, "patch", names);
    let parameter = patch.parameter;
    let changed = patch.changed;
    let assignments = patch.assignments;
    let present = columns
        .iter()
        .map(|column| format!("filter.{}.is_some()", names.field(table, &column.name)))
        .collect::<Vec<_>>()
        .join(" || ");
    let prefix = format!("UPDATE {} SET ", sql(&table.name));
    format!(
        "/// Updates rows matching any typed predicate group in one statement.\n\
         /// Fields within each group use AND; groups use OR. Values stay bound.\n\
         /// Empty groups or unconstrained groups fail before SQL; empty patches return zero.\n\
         /// Nullable patch Some(None) clears a column; omitted fields stay unchanged.\n\
         /// Returns affected rows; database failures propagate without committing.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, filter: {filter}, patch: {patch_type}) -> Result<(), sqlx::Error> {{\n\
         /// let affected = {function}(pool, vec![filter], patch).await?;\n\
         /// println!(\"updated: {{}}\", affected);\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, groups: Vec<{filter}>, {parameter}) -> Result<u64, sqlx::Error> {{\n\
             if groups.is_empty() || groups.iter().any(|filter| !({present})) {{\n\
                 return Err(sqlx::Error::Protocol(\"Filtered update requires constrained groups\".into()));\n\
             }}\n\
             if !({changed}) {{ return Ok(0); }}\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {{\n\
                 let mut set = query.separated(\", \");\n\
                 {assignments}\n\
             }}\n\
             query.push(\" WHERE FALSE\");\n\
             for filter in groups {{\n\
                 query.push(\" OR (TRUE\");\n\
                 {bindings}\n\
                 query.push(\")\");\n\
             }}\n\
             Ok(query.build().execute(executor).await?.rows_affected())\n\
         }}\n\n",
        patch_type = names.patch(table),
    )
}
