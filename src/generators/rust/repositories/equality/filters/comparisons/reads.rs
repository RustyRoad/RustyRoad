//! Matching row and count queries share one generated comparison contract.
use super::{keys, sql, Names, Table};

/// Emits a count and a bounded row read with identical predicate semantics.
/// Ordering columns and operators are validated; values and bounds stay bound.
/// Both operations accept a pool or a caller-owned PostgreSQL connection.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names, equality: &str, comparisons: &str) -> String {
    let model = names.row(table);
    let filter = names.filter(table);
    let input = names.comparison(table);
    let orders = keys(table)
        .iter()
        .map(|key| format!("{:?} => {:?},", key[0], sql(&key[0])))
        .collect::<Vec<_>>()
        .join("\n");
    let ordering = format!(
        "if let Some((column, ascending)) = order_by {{\n\
             let column = match column {{\n{orders}\n\
                 _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported order column: {{column}}\"))),\n\
             }};\n\
             query.push(\" ORDER BY \").push(column);\n\
             query.push(if ascending {{ \" ASC NULLS LAST\" }} else {{ \" DESC NULLS FIRST\" }});\n\
         }}\n\
         if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
         if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}"
    );
    let mut output = String::new();
    for (verb, result, projection, builder, fetch, bounded) in [
        (
            "filter_compare",
            format!("Vec<{model}>"),
            "*",
            format!("build_query_as::<{model}>()"),
            "fetch_all",
            true,
        ),
        (
            "filter_compare_count",
            "i64".into(),
            "COUNT(*)",
            "build_query_scalar::<i64>()".into(),
            "fetch_one",
            false,
        ),
    ] {
        let function = names.function(table, verb);
        let prefix = format!("SELECT {projection} FROM {} WHERE TRUE", sql(&table.name));
        let parameters = if bounded {
            ", order_by: Option<(&str, bool)>, limit: Option<i64>, offset: Option<i64>"
        } else {
            ""
        };
        let arguments = if bounded { ", None, None, None" } else { "" };
        let bounds = if bounded { ordering.as_str() } else { "" };
        output.push_str(&format!(
            "/// Reads rows or counts using typed AND comparisons plus equality filters.\n\
             /// Example: `{function}(pool, {filter}::default(), {input}::default(){arguments}).await?`.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}, comparisons: {input}{parameters}) -> Result<{result}, sqlx::Error> {{\n\
                 let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
                 {equality}\n{comparisons}\n{bounds}\n\
                 query.{builder}.{fetch}(executor).await\n\
             }}\n\n"
        ));
    }
    output
}
