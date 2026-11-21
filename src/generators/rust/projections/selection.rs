//! Shared ordered SQL selections and tuple types for reads and inserted rows.
use super::{Names, Projection};
use crate::database::introspection::{Schema, Table};

/// Derives selected expressions and tuple codecs from the same catalog columns.
/// Casts preserve database nullability and execute inside PostgreSQL.
/// # Examples
/// ```toml
/// [models.rust_projections.snapshot_insert]
/// table = "route_optimization_snapshots"
/// columns = ["id", "planned_total_cost"]
/// numeric_float = ["planned_total_cost"]
/// insert = true
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    table: &Table,
    projection: &Projection,
) -> (String, String) {
    let selected = projection
        .columns
        .iter()
        .map(|name| {
            super::casts::field(
                schema,
                names,
                table,
                table.column(name).unwrap(),
                projection,
            )
        })
        .collect::<Vec<_>>();
    let columns = selected
        .iter()
        .map(|(sql, _)| sql.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let types = selected
        .iter()
        .map(|(_, ty)| ty.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    (columns, format!("({types},)"))
}
