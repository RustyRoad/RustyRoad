use super::{Names, Projection};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{naming::sql, repositories::equality};

/// Emits a bound, optionally limited projection without decoding unrelated columns.
/// Tuple order follows the configured column order; no ordering is invented.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    table: &Table,
    projection: &Projection,
) -> String {
    let (columns, tuple) = super::selection::render(schema, names, table, projection);
    let filters: Vec<_> = equality::keys(table)
        .iter()
        .map(|key| table.column(&key[0]).unwrap())
        .collect();
    let bindings = equality::equality_bindings(table, &filters, names);
    let input = names.comparison(table);
    let (parameters, argument, comparisons) = if projection.comparisons {
        (
            format!(", comparisons: {input}"),
            format!(", {input}::default()"),
            equality::comparison_bindings(table, names, &filters),
        )
    } else {
        (String::new(), String::new(), String::new())
    };
    let filter = names.filter(table);
    let prefix = format!("SELECT {columns} FROM {} WHERE TRUE", sql(&table.name));
    let ordering = super::ordering::render(projection);
    format!(
        "/// Reads selected schema columns using typed predicates.\n\
         /// Tuple fields retain database nullability; configured database ordering is preserved.\n\
         /// Configured JSON text fields use PostgreSQL ::text, not Rust serialization.\n\
         /// Configured numeric floats use PostgreSQL ::float8, retaining database cast errors.\n\
         /// Network host selections use PostgreSQL HOST formatting without network prefixes.\n\
         /// Optional limits stay bound; errors propagate without committing the executor.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, filter: {filter}) -> Result<(), sqlx::Error> {{\n\
         /// let rows = {function}(pool, filter{argument}, Some(2)).await?;\n\
         /// println!(\"matching rows (up to two): {{}}\", rows.len());\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}{parameters}, limit: Option<i64>) -> Result<Vec<{tuple}>, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             {comparisons}\n\
             query.push({ordering:?});\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             query.build_query_as::<{tuple}>().fetch_all(executor).await\n\
         }}\n\n"
    )
}
