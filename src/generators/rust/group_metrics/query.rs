use super::{expressions, predicate, GroupMetrics, Names};
use crate::database::introspection::Schema;
use crate::generators::rust::{naming::sql, repositories::equality, types};

/// Builds grouped or single-row aggregates with typed filters and bound values.
/// Grouped SUM(CASE) semantics remain unchanged; global counts return zero on empty input.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    report: &GroupMetrics,
) -> String {
    let table = schema
        .tables
        .iter()
        .find(|t| t.name == report.table)
        .unwrap();
    let global = report.group_by.is_empty();
    let group = table.column(&report.group_by);
    let mut result_types: Vec<String> = group
        .iter()
        .map(|column| types::map_column(table, column, schema, names).rust)
        .collect();
    let mut metrics = String::new();
    for metric in &report.metrics {
        let separator = if result_types.is_empty() { "" } else { ", " };
        let (expression, result) =
            expressions::render(schema, table, names, metric, global, separator);
        metrics.push_str(&expression);
        result_types.push(result);
    }
    let tuple = format!("({},)", result_types.join(", "));
    let start = group
        .map(|column| format!("SELECT {}", sql(&column.name)))
        .unwrap_or_else(|| "SELECT ".into());
    let from = format!(" FROM {} WHERE TRUE", sql(&report.table));
    let finish = if global {
        String::new()
    } else {
        let group = sql(&report.group_by);
        format!(" GROUP BY {group} ORDER BY {group} ASC")
    };
    let filter = names.filter(table);
    let bindings = equality::aggregate_bindings(table, names);
    let (parameter, runtime_filter) =
        predicate::runtime(schema, table, report.filter_by.as_deref());
    let argument = if report.filter_by.is_some() {
        ", value"
    } else {
        ""
    };
    let result = if global {
        tuple.clone()
    } else {
        format!("Vec<{tuple}>")
    };
    let fetch = if global { "fetch_one" } else { "fetch_all" };
    format!(
        "/// Returns configured metrics in order, preceded by the group when configured.\n\
         /// Ungrouped reports return one row; counts are zero when no rows match.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool{parameter}) -> Result<(), sqlx::Error> {{\n\
         /// let metrics = {function}(pool, {filter}::default(){argument}).await?;\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}{parameter}) -> Result<{result}, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({start:?});\n\
             {metrics}\n\
             query.push({from:?});\n\
             {bindings}\n\
             {runtime_filter}\n\
             query.push({finish:?});\n\
             query.build_query_as::<{tuple}>().{fetch}(executor).await\n\
         }}\n\n"
    )
}
