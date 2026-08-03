//! Rendering one JSON Schema object.

use super::{fields, required, Variant};
use crate::database::introspection::{Schema, Table};
use crate::generators::typescript::casing::Casing;

/// Renders one schema object.
pub(super) fn render(
    name: &str,
    schema: &Schema,
    table: &Table,
    casing: Casing,
    variant: Variant,
) -> String {
    let required = required::required(table, casing, variant);
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
        fields::properties(schema, table, casing, variant)
    )
}

/// Renders the shared error schema, referenced by every 404.
pub(super) fn error_schema() -> String {
    "      \"ApiError\": {\n\
     \x20       \"type\": \"object\",\n\
     \x20       \"properties\": { \"error\": { \"type\": \"string\" } },\n\
     \x20       \"required\": [\"error\"]\n\
     \x20     }"
        .to_string()
}
