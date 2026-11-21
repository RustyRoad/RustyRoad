//! One row per group, chosen by explicit PostgreSQL ordering in one statement.
use super::{keys, sql, Names, Table};

/// Emits a DISTINCT ON read over catalog-validated scalar columns.
/// Group order is ascending; callers specify secondary direction and NULL position.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, names: &Names, bindings: &str) -> String {
    let function = names.function(table, "filter_distinct_first");
    let filter = names.filter(table);
    let row = names.row(table);
    let columns = keys(table);
    let example = &columns[0][0];
    let allowed = columns
        .iter()
        .map(|key| format!("{:?} => {:?},", key[0], sql(&key[0])))
        .collect::<Vec<_>>()
        .join("\n");
    let from = format!(") * FROM {} WHERE TRUE", sql(&table.name));
    format!(
        "/// Selects the first matching row in each group using explicit secondary ordering.\n\
         /// Ties remain unspecified unless the caller includes a unique tie-breaker.\n\
         /// Example: `{function}(pool, {filter}::default(), {example:?}, &[], None, None).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}, group_by: &str, order_by: &[(&str, bool, bool)], limit: Option<i64>, offset: Option<i64>) -> Result<Vec<{row}>, sqlx::Error> {{\n\
             let group = match group_by {{\n{allowed}\n\
                 _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported group column: {{group_by}}\"))),\n\
             }};\n\
             let mut order = Vec::new();\n\
             for &(column, ascending, nulls_first) in order_by {{\n\
                 let column = match column {{\n{allowed}\n\
                     _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported order column: {{column}}\"))),\n\
                 }};\n\
                 order.push((column, ascending, nulls_first));\n\
             }};\n\
             let mut query = QueryBuilder::<Postgres>::new(\"SELECT DISTINCT ON (\");\n\
             query.push(group).push({from:?});\n\
             {bindings}\n\
             query.push(\" ORDER BY \").push(group).push(\" ASC NULLS LAST\");\n\
             for (column, ascending, nulls_first) in order {{\n\
                 query.push(\", \").push(column);\n\
                 query.push(if ascending {{ \" ASC\" }} else {{ \" DESC\" }});\n\
                 query.push(if nulls_first {{ \" NULLS FIRST\" }} else {{ \" NULLS LAST\" }});\n\
             }}\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}\n\
             query.build_query_as::<{row}>().fetch_all(executor).await\n\
         }}\n\n"
    )
}
