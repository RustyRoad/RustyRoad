//! Catalog-checked parameter codecs; RETURNING casts never imply input casts.
use super::configuration::InsertInput;
use crate::database::introspection::{Column, Table};
use std::collections::BTreeSet;

/// Reject unknown, repeated, identity, or incompatible input cast targets.
/// Casts may target writable columns that are not in the returned tuple.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(table: &Table, input: &InsertInput) -> Result<(), &'static str> {
    let mut seen = BTreeSet::new();
    for name in input.numeric_float.iter().chain(&input.network_text) {
        if !seen.insert(name) {
            return Err("duplicate input cast column");
        }
        let column = table.column(name).ok_or("unknown input cast column")?;
        let kind = column.sql_type.trim().to_ascii_lowercase();
        let compatible = if input.numeric_float.contains(name) {
            super::super::casts::numeric(&kind)
        } else {
            matches!(kind.as_str(), "inet" | "cidr")
        };
        if column.auto_increment || !compatible {
            return Err("input casts require writable compatible scalar columns");
        }
    }
    Ok(())
}

/// Return the explicit parameter codec and its trusted SQL cast suffix.
/// Double casts establish the binary bind type before converting to storage type.
/// Unlisted inputs retain their normal catalog/project codec, even if outputs cast.
/// # Examples
/// ```toml
/// [models.rust_projections.insert_snapshot]
/// table = "route_optimization_snapshots"
/// columns = ["id"]
/// insert = true
/// insert_input = { numeric_float = ["planned_total_cost"] }
/// ```
pub(super) fn cast(column: &Column, input: &InsertInput) -> Option<(&'static str, &'static str)> {
    if input.numeric_float.contains(&column.name) {
        Some(("f64", "::float8::numeric"))
    } else if input.network_text.contains(&column.name) {
        let suffix = if column.sql_type.trim().eq_ignore_ascii_case("cidr") {
            "::text::cidr"
        } else {
            "::text::inet"
        };
        Some(("String", suffix))
    } else {
        None
    }
}
