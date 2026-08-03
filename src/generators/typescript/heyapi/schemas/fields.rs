//! Property maps for a schema variant.

use super::required::key;
use super::types;
use super::Variant;
use crate::database::introspection::{Schema, Table};
use crate::generators::typescript::casing::Casing;

/// Renders the property map for a variant.
pub(super) fn properties(
    schema: &Schema,
    table: &Table,
    casing: Casing,
    variant: Variant,
) -> String {
    table
        .columns
        .iter()
        // A generated key is assigned by the database, so a client cannot supply it.
        .filter(|column| variant == Variant::Select || !column.auto_increment)
        .map(|column| {
            format!(
                "            \"{}\": {}",
                key(&column.name, casing),
                types::property(schema, column)
            )
        })
        .collect::<Vec<_>>()
        .join(",\n")
}
