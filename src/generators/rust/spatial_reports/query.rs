use super::{columns, Names, SpatialReport};
use crate::database::introspection::Schema;
use crate::generators::rust::{naming::sql, repositories::equality};

/// Emits one filtered, ordered spatial join with optional projection grouping.
/// Geometry remains server-side and is returned through SQLx's JSON codec.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    report: &SpatialReport,
) -> String {
    let source = schema
        .tables
        .iter()
        .find(|t| t.name == report.source_table)
        .unwrap();
    let (mut selected, mut mapped) = columns::projection(schema, names, report);
    let geometry = format!("\"related_row\".{}", sql(&report.geometry));
    let mut grouped = selected.clone();
    grouped.push(geometry.clone());
    selected.push(format!("ST_AsGeoJSON({geometry})::json"));
    mapped.push("Option<serde_json::Value>".into());
    let tuple = format!("({},)", mapped.join(", "));
    let filter = names.filter(source);
    let bindings = equality::qualified_aggregate_bindings(source, names, "source_row");
    let prefix = format!("SELECT {} FROM {} AS \"source_row\" LEFT JOIN {} AS \"related_row\" ON \"source_row\".{} = \"related_row\".{} WHERE {geometry} IS NOT NULL",
        selected.join(", "), sql(&report.source_table), sql(&report.target_table),
        sql(&report.source_key), sql(&report.target_key));
    let grouping = if report.group_rows {
        format!(" GROUP BY {}", grouped.join(", "))
    } else {
        String::new()
    };
    let direction = if report.descending { "DESC" } else { "ASC" };
    let suffix = format!(
        "{grouping} ORDER BY \"source_row\".{} {direction}",
        sql(&report.order_by)
    );
    format!(
        "/// Returns configured source columns, nullable target columns, and GeoJSON in that order.\n\
         /// Example: `{function}(pool, {filter}::default()).await?`.\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}) -> Result<Vec<{tuple}>, sqlx::Error> {{\n\
             let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
             {bindings}\n\
             query.push({suffix:?});\n\
             query.build_query_as::<{tuple}>().fetch_all(executor).await\n\
         }}\n\n"
    )
}
