//! Ordered, bounded reads with generated multi-column equality predicates.
use super::{keys, sql, Names, Table};

/// Render ordering over an explicit filter, including an empty filter.
/// Catalog columns form an allowlist; values, limits and offsets stay bound.
/// Ascending and descending preserve PostgreSQL's default null ordering.
/// Generated reads reuse the supplied pool or transaction connection and never commit it.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names, bindings: &str) -> String {
    let model = names.row(table);
    let filter = names.filter(table);
    let function = names.function(table, "filter_ordered");
    let columns = keys(table);
    let example_column = &columns[0][0];
    let order_columns = columns
        .iter()
        .map(|columns| {
            let column = &columns[0];
            format!("{column:?} => {:?},", sql(column))
        })
        .collect::<Vec<_>>()
        .join("\n");
    let prefix = format!("SELECT * FROM {} WHERE TRUE", sql(&table.name));
    format!(
        "/// Apply typed predicates and catalog-validated ordering with optional bounds.\n\
         /// Example: `{function}(pool, {filter}::default(), {example_column:?}, false, Some(1), None).await`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}, order_by: &str, ascending: bool, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             let order = match order_by {{\n{order_columns}\n\
                 _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported order column: {{order_by}}\"))),\n\
             }};\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             query.push(\" ORDER BY \").push(order);\n\
             query.push(if ascending {{ \" ASC NULLS LAST\" }} else {{ \" DESC NULLS FIRST\" }});\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}\n\
             query.build_query_as::<{model}>().fetch_all(executor).await\n\
         }}\n\n"
    )
}
