use super::Names;
use crate::database::introspection::{Column, Schema, Table};
use crate::generators::rust::{naming::sql, repositories::equality, types};

/// Emits grouped COUNT/MIN/MAX with typed filters in one statement snapshot.
/// The returned tuple retains the group codec and nullable timestamp ranges.
/// # Examples
/// ```toml
/// [models.rust_group_stats.campaign_scan_summary]
/// table = "scan_events"
/// group_by = "campaign"
/// timestamp = "scanned_at"
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    table: &Table,
    group: &Column,
    timestamp: &Column,
) -> String {
    let group_type = types::map_column(table, group, schema, names).rust;
    let mut nullable_timestamp = timestamp.clone();
    nullable_timestamp.nullable = true;
    let range_type = types::map_column(table, &nullable_timestamp, schema, names).rust;
    let filter = names.filter(table);
    let group = sql(&group.name);
    let timestamp = sql(&timestamp.name);
    let prefix = format!(
        "SELECT {group}, COUNT(*)::bigint, MIN({timestamp}), MAX({timestamp}) FROM {} WHERE TRUE",
        sql(&table.name)
    );
    let suffix = format!(" GROUP BY {group} ORDER BY COUNT(*) DESC, {group} ASC");
    let bindings = equality::aggregate_bindings(table, names);
    format!(
        "/// Groups matching rows, ordered by count descending and group ascending.\n\
         /// Returns group, count, first timestamp, last timestamp; empty tables return no rows.\n\
         /// Example: `{function}(pool, {filter}::default()).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}) -> Result<Vec<({group_type}, i64, {range_type}, {range_type})>, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             query.push({suffix:?});\n\
             query.build_query_as::<({group_type}, i64, {range_type}, {range_type})>().fetch_all(executor).await\n\
         }}\n\n"
    )
}
