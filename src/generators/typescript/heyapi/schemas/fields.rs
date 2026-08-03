//! Property maps and required lists for a schema variant.

use super::types;
use crate::database::introspection::Table;
use crate::generators::typescript::casing::{identifier, Casing};

/// Which schema variant is being rendered.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Variant {
    /// Rows returned by the API.
    Select,
    /// Rows accepted on create.
    Insert,
    /// Rows accepted on partial update.
    Patch,
}

/// Renders the property map for a variant.
pub(super) fn properties(table: &Table, casing: Casing, variant: Variant) -> String {
    table
        .columns
        .iter()
        // A generated key is assigned by the database, so a client cannot supply it.
        .filter(|column| variant == Variant::Select || !column.auto_increment)
        .map(|column| {
            format!(
                "            \"{}\": {}",
                key(&column.name, casing),
                types::property(column)
            )
        })
        .collect::<Vec<_>>()
        .join(",\n")
}

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
fn is_required(
    column: &crate::database::introspection::Column,
    variant: Variant,
) -> bool {
    match variant {
        Variant::Select => true,
        // A default or generated value may be omitted by the client.
        _ => !column.auto_increment && column.default.is_none(),
    }
}

/// Returns a column's JSON key, without the quoting an object literal would need.
fn key(name: &str, casing: Casing) -> String {
    identifier(name, casing).trim_matches('"').to_string()
}
