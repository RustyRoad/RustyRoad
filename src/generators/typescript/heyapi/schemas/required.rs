//! Required-field lists for a schema variant.

use super::Variant;
use crate::database::introspection::{Column, Table};
use crate::generators::typescript::casing::{identifier, Casing};

/// Renders the required list for a variant.
pub(super) fn required(table: &Table, casing: Casing, variant: Variant) -> String {
    if variant == Variant::Patch {
        // Every field is optional on a partial update.
        return String::new();
    }

    table
        .columns
        .iter()
        .filter(|column| !column.nullable && is_required(column, variant))
        .map(|column| format!("\"{}\"", key(&column.name, casing)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Returns `true` when a non-nullable column must be supplied for this variant.
fn is_required(column: &Column, variant: Variant) -> bool {
    match variant {
        Variant::Select => true,
        // A default or generated value may be omitted by the client.
        _ => !column.auto_increment && column.default.is_none(),
    }
}

/// Returns a column's JSON key, without the quoting an object literal would need.
pub(super) fn key(name: &str, casing: Casing) -> String {
    identifier(name, casing).trim_matches('"').to_string()
}
