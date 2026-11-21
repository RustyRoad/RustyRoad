use super::super::{Names, Projection};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{naming::sql, types};

/// Selects only configured columns and derives each tuple element from its owning table.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn selected(
    schema: &Schema,
    names: &Names,
    table: &Table,
    related: &Table,
    projection: &Projection,
) -> (String, String) {
    let join = projection.join.as_ref().unwrap();
    let mut columns = Vec::new();
    let mut fields = Vec::new();
    for (owner, alias, selection) in [
        (table, "source_row", &projection.columns),
        (related, "related_row", &join.columns),
    ] {
        for name in selection {
            let column = owner.column(name).unwrap();
            columns.push(format!("{}.{}", sql(alias), sql(name)));
            fields.push(types::map_column(owner, column, schema, names).rust);
        }
    }
    (columns.join(", "), format!("({},)", fields.join(", ")))
}

/// Keeps configured related-column precedence, direction and optional NULL placement.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn ordering(projection: &Projection) -> String {
    let join = projection.join.as_ref().unwrap();
    let orders: Vec<_> = join
        .order
        .iter()
        .map(|order| {
            let direction = if order.descending { "DESC" } else { "ASC" };
            let nulls = match order.nulls_first {
                Some(true) => " NULLS FIRST",
                Some(false) => " NULLS LAST",
                None => "",
            };
            format!("related_row.{} {direction}{nulls}", sql(&order.column))
        })
        .collect();
    if orders.is_empty() {
        String::new()
    } else {
        format!(" ORDER BY {}", orders.join(", "))
    }
}
