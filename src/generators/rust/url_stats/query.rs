use super::{Names, UrlStats};
use crate::database::introspection::Schema;
use crate::generators::rust::{naming::sql, repositories::equality, types};

/// Emits two LEFT JOINs, composite-row counts and a strict transaction-clock window.
/// Zero-event sources and duplicate-link multiplicity remain visible.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(schema: &Schema, names: &Names, function: &str, report: &UrlStats) -> String {
    let table = schema.table(&report.table).unwrap();
    let key_type = types::map_column(table, table.column(&report.key).unwrap(), schema, names).rust;
    let label_type =
        types::map_column(table, table.column(&report.label).unwrap(), schema, names).rust;
    let filter = names.filter(table);
    let bindings = equality::qualified_aggregate_bindings(table, names, "source_row");
    let key = format!("source_row.{}", sql(&report.key));
    let label = format!("source_row.{}", sql(&report.label));
    let prefix = format!("SELECT {key}, {label}, COUNT(event_row.*), COUNT(event_row.*) FILTER         (WHERE event_row.{} > CURRENT_TIMESTAMP - INTERVAL '{} HOURS')         FROM {} AS source_row LEFT JOIN {} AS link_row ON link_row.{} LIKE CONCAT(",
        sql(&report.timestamp), report.window_hours, sql(&report.table), sql(&report.links), sql(&report.url));
    let middle = format!(", {key}::text, ");
    let suffix = format!(
        ") LEFT JOIN {} AS event_row ON event_row.{} = link_row.{} WHERE TRUE",
        sql(&report.events),
        sql(&report.event_key),
        sql(&report.link_key)
    );
    let finish = format!(" GROUP BY {key}, {label} ORDER BY COUNT(event_row.*) DESC, {key} ASC");
    format!(
        "/// Returns source identity, label, all-event count and recent-event count.\n\
         /// Retains sources with no matching events; ordered by count descending and key ascending.\n\
         /// LIKE wildcards and join multiplicity are preserved; query errors propagate.\n\
         /// Example: `{function}(pool, {filter}::default()).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}) -> Result<Vec<({key_type}, {label_type}, i64, i64)>, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             query.push_bind({url_prefix:?}).push({middle:?}).push_bind({url_suffix:?}).push({suffix:?});\n\
             {bindings}\n\
             query.push({finish:?});\n\
             query.build_query_as::<({key_type}, {label_type}, i64, i64)>().fetch_all(executor).await\n\
         }}\n\n",
        url_prefix = report.prefix, url_suffix = report.suffix,
    )
}
