use super::super::{Names, Projection};
use super::fields;
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{naming::sql, repositories::equality};

/// Emits a bound foreign-key inner join without decoding unselected columns.
/// Tuple order is source selection then related selection; ordering follows configuration.
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
    let join = projection.join.as_ref().unwrap();
    let foreign = table
        .foreign_keys
        .iter()
        .find(|key| key.name == join.foreign_key)
        .unwrap();
    let related = schema.table(&foreign.foreign_table).unwrap();
    let (columns, tuple) = fields::selected(schema, names, table, related, projection);
    let predicate = foreign
        .columns
        .iter()
        .zip(&foreign.foreign_columns)
        .map(|(local, remote)| format!("source_row.{} = related_row.{}", sql(local), sql(remote)))
        .collect::<Vec<_>>()
        .join(" AND ");
    let filter = names.filter(table);
    let bindings = equality::qualified_aggregate_bindings(table, names, "source_row");
    let prefix = format!(
        "SELECT {columns} FROM {} AS source_row JOIN {} AS related_row ON {predicate} WHERE TRUE",
        sql(&table.name),
        sql(&related.name)
    );
    let ordering = fields::ordering(projection);
    format!(
        "/// Reads selected schema columns using typed equality predicates.\n\
         /// Tuple fields retain database nullability; ordering follows the configured related columns.\n\
         /// Optional limits stay bound; errors propagate without committing the executor.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, filter: {filter}) -> Result<(), sqlx::Error> {{\n\
         /// let rows = {function}(pool, filter, Some(2)).await?;\n\
         /// println!(\"matching rows (up to two): {{}}\", rows.len());\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}, limit: Option<i64>) -> Result<Vec<{tuple}>, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             query.push({ordering:?});\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             query.build_query_as::<{tuple}>().fetch_all(executor).await\n\
         }}\n\n"
    )
}
