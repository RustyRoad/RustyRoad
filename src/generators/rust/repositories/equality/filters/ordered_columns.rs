//! Catalog-validated multi-column ordering over generated equality filters.
use super::{keys, sql, Names, Table};

/// Emits ordered reads with explicit precedence and optional per-column NULL order.
/// Caller-supplied identifiers are matched to quoted catalog literals before use.
/// Limits and offsets remain bound; the supplied executor owns the transaction.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, names: &Names, bindings: &str) -> String {
    let model = names.row(table);
    let filter = names.filter(table);
    let function = names.function(table, "filter_ordered_columns");
    let columns = keys(table);
    let example = &columns[0][0];
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
        "/// Reads matching rows with caller-selected ordering precedence.\n\
         /// Each tuple is (column, ascending, nulls_first); None uses PostgreSQL defaults.\n\
         /// Empty or unknown ordering is rejected before executing SQL. Bounds stay bound.\n\
         /// No implicit tie-breaker or transaction commit is added.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, filter: {filter}) -> Result<(), sqlx::Error> {{\n\
         /// let rows = {function}(pool, filter, &[({example:?}, true, None)], None, None).await?;\n\
         /// println!(\"matching rows: {{}}\", rows.len());\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}, order_by: &[(&str, bool, Option<bool>)], limit: Option<i64>, offset: Option<i64>) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             if order_by.is_empty() {{ return Err(sqlx::Error::Protocol(\"Ordered reads require an order column\".into())); }}\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             query.push(\" ORDER BY \");\n\
             for (index, &(column, ascending, nulls_first)) in order_by.iter().enumerate() {{\n\
                 let order = match column {{\n{order_columns}\n\
                     _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported order column: {{column}}\"))),\n\
                 }};\n\
                 if index > 0 {{ query.push(\", \"); }}\n\
                 query.push(order).push(if ascending {{ \" ASC\" }} else {{ \" DESC\" }});\n\
                 if let Some(first) = nulls_first {{ query.push(if first {{ \" NULLS FIRST\" }} else {{ \" NULLS LAST\" }}); }}\n\
             }}\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}\n\
             query.build_query_as::<{model}>().fetch_all(executor).await\n\
         }}\n\n"
    )
}
