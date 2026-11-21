//! Catalog-validated ordering for configured unjoined projection reads.
use super::Projection;
use crate::database::introspection::Table;
use crate::generators::rust::naming::sql;
use std::collections::BTreeSet;

/// Require known distinct columns and reject root ordering on joined projections.
/// Joined projections retain their separately configured related-column ordering.
/// # Examples
/// ```toml
/// [models.rust_projections.latest_snapshot]
/// table = "route_optimization_snapshots"
/// columns = ["id", "created_at"]
/// order = [{column = "created_at", descending = true}]
/// ```
pub(super) fn validate(table: &Table, projection: &Projection) -> Result<(), &'static str> {
    let mut seen = BTreeSet::new();
    if (!projection.order.is_empty() && projection.join.is_some())
        || projection
            .order
            .iter()
            .any(|order| table.column(&order.column).is_none() || !seen.insert(&order.column))
    {
        return Err("projection order requires distinct known columns without a join");
    }
    Ok(())
}

/// Quote catalog identifiers and retain explicit direction and NULL placement.
/// No tie breaker is added when the caller did not configure one.
/// # Examples
/// ```toml
/// [models.rust_projections.latest_snapshot]
/// table = "route_optimization_snapshots"
/// columns = ["id", "created_at"]
/// order = [{column = "created_at", descending = true}]
/// ```
pub(super) fn render(projection: &Projection) -> String {
    let orders = projection
        .order
        .iter()
        .map(|order| {
            let direction = if order.descending { "DESC" } else { "ASC" };
            let nulls = match order.nulls_first {
                Some(true) => " NULLS FIRST",
                Some(false) => " NULLS LAST",
                None => "",
            };
            format!("{} {direction}{nulls}", sql(&order.column))
        })
        .collect::<Vec<_>>();
    if orders.is_empty() {
        String::new()
    } else {
        format!(" ORDER BY {}", orders.join(", "))
    }
}
