//! Server-clock predicates combined with generated equality filters.
use super::{sql, Names, Table};
use crate::database::introspection::Column;
mod read;

/// Emits clock comparisons and optional ordered bounds using catalog allowlists.
/// Multiple comparisons share PostgreSQL transaction time, not application time.
/// Generated reads reuse the supplied pool or transaction connection and never commit it.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names, columns: &[&Column], bindings: &str) -> String {
    let temporal: Vec<_> = columns
        .iter()
        .filter(|column| super::clock_delete::timestamp(&column.sql_type))
        .collect();
    if temporal.is_empty() {
        return String::new();
    }
    let clock_columns = temporal
        .iter()
        .map(|column| format!("{:?} => {:?},", column.name, sql(&column.name)))
        .collect::<Vec<_>>()
        .join("\n");
    let order_columns = columns
        .iter()
        .map(|column| format!("{:?} => {:?},", column.name, sql(&column.name)))
        .collect::<Vec<_>>()
        .join("\n");
    let mut output = String::new();
    for (verb, include_null) in [("filter_clock", false), ("filter_clock_or_null", true)] {
        output.push_str(&read::render(
            table,
            names,
            bindings,
            &clock_columns,
            &order_columns,
            verb,
            include_null,
        ));
    }
    output
}
