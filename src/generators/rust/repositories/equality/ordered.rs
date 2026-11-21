//! Server-side ordering and optional bounds over one generated equality selector.
use super::{keys, selector, sql, Names, Schema, Table};

/// Emit a bounded, ordered equality read using a catalog column allowlist.
/// Ascending uses PostgreSQL NULLS LAST, descending uses NULLS FIRST. Negative
/// limits/offsets retain PostgreSQL errors; absent bounds are omitted entirely.
/// Generated reads reuse the supplied pool or transaction connection and never commit it.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, schema: &Schema, key: &[String], names: &Names) -> String {
    let model = names.row(table);
    let function = names.selector(table, "ordered", key);
    let selector::Selector {
        parameters,
        arguments,
        ..
    } = selector::render(table, schema, key, names);
    let order_columns = keys(table)
        .iter()
        .map(|columns| {
            let column = &columns[0];
            format!("{column:?} => {:?},", sql(column))
        })
        .collect::<Vec<_>>()
        .join("\n");
    let prefix = format!(
        "SELECT * FROM {} WHERE {} = ",
        sql(&table.name),
        sql(&key[0])
    );
    format!(
        "/// Reads matching rows with catalog-validated ordering and optional bounds.\n\
         /// Example: `{function}(pool, {arguments}, {column:?}, true, Some(50), Some(0)).await`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {parameters}, order_by: &str, ascending: bool, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             let order = match order_by {{\n{order_columns}\n\
                 _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported order column: {{order_by}}\"))),\n\
             }};\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             query.push_bind({arguments}).push(\" ORDER BY \").push(order);\n\
             query.push(if ascending {{ \" ASC NULLS LAST\" }} else {{ \" DESC NULLS FIRST\" }});\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}\n\
             query.build_query_as::<{model}>().fetch_all(executor).await\n\
         }}\n\n",
        column = key[0],
    )
}
