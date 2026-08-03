//! `pgEnum` declaration rendering.
//!
//! A Postgres enum reaches introspection as a bare type name, so without this a
//! column degrades to `text` and the allowed values are lost from the types, the
//! validation, and the OpenAPI document.

use crate::database::introspection::Schema;
use crate::generators::typescript::casing::{binding, Casing};

/// Renders the `pgEnum` declarations for a schema.
///
/// Declared before the tables that reference them, since a `const` is not hoisted.
pub(super) fn render(schema: &Schema, casing: Casing) -> String {
    if schema.enums.is_empty() {
        return String::new();
    }

    let declarations = schema
        .enums
        .iter()
        .map(|item| {
            let values = item
                .values
                .iter()
                .map(|value| format!("\"{}\"", value.replace('"', "\\\"")))
                .collect::<Vec<_>>()
                .join(", ");

            format!(
                "export const {} = pgEnum(\"{}\", [{values}]);",
                binding(&item.name, casing),
                item.name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("{declarations}\n\n")
}

/// Returns the enum whose name matches `sql_type`, if any.
///
/// `format_type` reports an enum column as its bare type name, which is how a
/// column is matched back to its declaration.
pub(super) fn lookup<'a>(
    schema: &'a Schema,
    sql_type: &str,
) -> Option<&'a crate::database::introspection::Enum> {
    let name = sql_type.trim().trim_matches('"');
    schema.enums.iter().find(|item| item.name == name)
}
