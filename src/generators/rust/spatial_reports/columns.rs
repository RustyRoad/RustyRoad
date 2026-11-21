use super::{Names, SpatialReport};
use crate::database::introspection::Schema;
use crate::generators::rust::{naming::sql, types};

/// Derives scalar tuple types and qualified projections in configured order.
/// Target columns remain optional because the query uses a left join.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn projection(
    schema: &Schema,
    names: &Names,
    report: &SpatialReport,
) -> (Vec<String>, Vec<String>) {
    let mut selected = Vec::new();
    let mut mapped = Vec::new();
    for (table, columns, alias, nullable) in [
        (
            &report.source_table,
            &report.source_columns,
            "source_row",
            false,
        ),
        (
            &report.target_table,
            &report.target_columns,
            "related_row",
            true,
        ),
    ] {
        let table = schema.tables.iter().find(|t| &t.name == table).unwrap();
        for name in columns {
            let mut column = table.column(name).unwrap().clone();
            column.nullable |= nullable;
            selected.push(format!("{}.{}", sql(alias), sql(name)));
            mapped.push(types::map_column(table, &column, schema, names).rust);
        }
    }
    (selected, mapped)
}
