use super::{predicate, ConditionalMetric};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::naming::sql;
use crate::generators::rust::{allocation::Names, types};

/// Renders one metric expression and its SQLx result type without executing SQL.
/// Conditional values remain parameters; sums retain NULL on empty input.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    table: &Table,
    names: &Names,
    metric: &ConditionalMetric,
    global: bool,
    separator: &str,
) -> (String, String) {
    match metric {
        ConditionalMetric::CountAll => {
            let expression = format!("{separator}COUNT(*)::bigint");
            (format!("query.push({expression:?});\n"), "i64".into())
        }
        ConditionalMetric::MaxTimestamp { column } => {
            let expression = format!("{separator}MAX({})", sql(column));
            let mut column = table.column(column).unwrap().clone();
            column.nullable = true;
            (
                format!("query.push({expression:?});\n"),
                types::map_column(table, &column, schema, names).rust,
            )
        }
        ConditionalMetric::Count { column, equals } if global => {
            let start = format!("{separator}COUNT(*) FILTER (WHERE {} = ", sql(column));
            let cast = predicate::cast(schema, table.column(column).unwrap()).unwrap();
            (bound(&start, equals, &format!("{cast})")), "i64".into())
        }
        ConditionalMetric::Count { column, equals } => {
            let start = format!("{separator}SUM(CASE WHEN {} = ", sql(column));
            let cast = predicate::cast(schema, table.column(column).unwrap()).unwrap();
            (
                bound(
                    &start,
                    equals,
                    &format!("{cast} THEN 1 ELSE 0 END)::bigint"),
                ),
                "Option<i64>".into(),
            )
        }
        ConditionalMetric::Sum {
            column,
            equals,
            value,
        } => {
            let start = format!("{separator}SUM(CASE WHEN {} = ", sql(column));
            let cast = predicate::cast(schema, table.column(column).unwrap()).unwrap();
            let end = format!(
                "{cast} THEN COALESCE({}, 0) ELSE 0 END)::double precision",
                sql(value)
            );
            (bound(&start, equals, &end), "Option<f64>".into())
        }
    }
}

/// Generates builder statements that bind a configured comparison value.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
fn bound(start: &str, value: &str, end: &str) -> String {
    format!("query.push({start:?}).push_bind({value:?}).push({end:?});\n")
}
