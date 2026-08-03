//! JSON Schema components for the OpenAPI document.
//!
//! Hey API generates its TypeScript types from `components.schemas`, so each table
//! needs a select, insert, and patch schema, plus a shared error shape.

mod fields;
mod types;

use crate::database::introspection::{Schema, Table};
use crate::generators::typescript::casing::{to_pascal, Casing};
use fields::Variant;

/// Renders the `components.schemas` object body.
pub(super) fn render(schema: &Schema, casing: Casing) -> String {
    let mut entries = vec![error_schema()];

    for table in &schema.tables {
        let name = to_pascal(&table.name);
        entries.push(object(&name, table, casing, Variant::Select));
        entries.push(object(&format!("New{name}"), table, casing, Variant::Insert));
        entries.push(object(&format!("Patch{name}"), table, casing, Variant::Patch));
    }

    entries.join(",\n")
}

/// Renders the shared error schema, referenced by every 404.
fn error_schema() -> String {
    "      \"ApiError\": {\n\
     \x20       \"type\": \"object\",\n\
     \x20       \"properties\": { \"error\": { \"type\": \"string\" } },\n\
     \x20       \"required\": [\"error\"]\n\
     \x20     }"
        .to_string()
}

/// Renders one schema object.
fn object(name: &str, table: &Table, casing: Casing, variant: Variant) -> String {
    let required = fields::required(table, casing, variant);
    let required_line = if required.is_empty() {
        String::new()
    } else {
        format!(",\n          \"required\": [{required}]")
    };

    format!(
        "      \"{name}\": {{\n\
         \x20         \"type\": \"object\",\n\
         \x20         \"properties\": {{\n{}\n          }}{required_line},\n\
         \x20         \"additionalProperties\": false\n\
         \x20     }}",
        fields::properties(table, casing, variant)
    )
}
