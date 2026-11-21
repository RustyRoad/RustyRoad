//! Render the validated clock read variants with distinct NULL semantics.
use super::{sql, Names, Table};

/// Emit either ordinary comparisons or parenthesized NULL-inclusive comparisons.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    table: &Table,
    names: &Names,
    bindings: &str,
    clock_columns: &str,
    order_columns: &str,
    verb: &str,
    include_null: bool,
) -> String {
    let predicate = if include_null {
        "query.push(\" AND (\").push(column).push(\" IS NULL OR \").push(column).push(comparison).push(\"CURRENT_TIMESTAMP)\");"
    } else {
        "query.push(\" AND \").push(column).push(comparison).push(\"CURRENT_TIMESTAMP\");"
    };
    let function = names.function(table, verb);
    let filter = names.filter(table);
    let model = names.row(table);
    let prefix = format!("SELECT * FROM {} WHERE TRUE", sql(&table.name));
    let null_rule = if include_null {
        "Each clock predicate also matches SQL NULL."
    } else {
        "SQL NULL timestamps do not match comparisons."
    };
    format!(
        "/// Applies equality and timestamp comparisons against PostgreSQL transaction time.\n\
         /// Clock columns, comparison operators and ordering are validated before execution.\n\
         /// {null_rule}\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}, clocks: &[(&str, &str)], order_by: Option<(&str, bool)>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<{model}>, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             for &(column, comparison) in clocks {{\n\
                 let column = match column {{\n{clock_columns}\n\
                     _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported clock column: {{column}}\"))),\n\
                 }};\n\
                 let comparison = match comparison {{\n\
                     \"<\" => \" < \", \"<=\" => \" <= \", \"=\" => \" = \",\n\
                     \">=\" => \" >= \", \">\" => \" > \",\n\
                     _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported clock comparison: {{comparison}}\"))),\n\
                 }};\n\
                 {predicate}\n\
             }}\n\
             {bindings}\n\
             if let Some((column, ascending)) = order_by {{\n\
                 let column = match column {{\n{order_columns}\n\
                     _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported order column: {{column}}\"))),\n\
                 }};\n\
                 query.push(\" ORDER BY \").push(column);\n\
                 query.push(if ascending {{ \" ASC NULLS LAST\" }} else {{ \" DESC NULLS FIRST\" }});\n\
             }}\n\
             if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
             if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}\n\
             query.build_query_as::<{model}>().fetch_all(executor).await\n\
         }}\n\n"
    )
}
