//! JSON Schema components for the OpenAPI document.
//!
//! Hey API generates its TypeScript types from `components.schemas`, so each table
//! needs a select, insert, and patch schema, plus a shared error shape.

mod fields;
mod object;
mod required;
mod types;

use crate::database::introspection::Schema;
use crate::generators::typescript::casing::{to_pascal, Casing};

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

/// Renders the `components.schemas` object body.
pub(super) fn render(schema: &Schema, casing: Casing) -> String {
    let mut entries = vec![object::error_schema()];

    for table in &schema.tables {
        let name = to_pascal(&table.name);
        for (prefix, variant) in [
            (String::new(), Variant::Select),
            ("New".to_string(), Variant::Insert),
            ("Patch".to_string(), Variant::Patch),
        ] {
            entries.push(object::render(
                &format!("{prefix}{name}"),
                schema,
                table,
                casing,
                variant,
            ));
        }
    }

    entries.join(",\n")
}
