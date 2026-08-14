//! The row's shape: its column list and its seeded defaults.

use crate::generators::tetherscript::model::{Field, Model};
use std::fmt::Write as _;

/// Renders the column accessor.
pub(super) fn columns(model: &Model) -> String {
    let listed = model
        .fields
        .iter()
        .map(|field| format!("        \"{}\"", field.column))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        "// Every column, in the order the database reports them.\n\
         fn columns() {{\n\
         \x20   return Ok([\n{listed}\n\x20   ])\n\
         }}\n\n"
    )
}

/// Renders `new`, seeding a row with a value for every column.
///
/// Every key is present rather than only the ones a caller sets, so reading a column
/// that was never assigned gives its zero instead of an absent-key surprise.
pub(super) fn new(model: &Model) -> String {
    let mut body = String::new();

    for field in &model.fields {
        let _ = writeln!(body, "    row[\"{}\"] = {}", field.column, seed(field));
    }

    format!(
        "// Builds a row with every column present.\n\
         //\n\
         // Database-assigned columns stay nil so validation can distinguish a value\n\
         // that was never supplied from one deliberately left empty.\n\
         fn new() {{\n\
         \x20   let mut row = map()\n{body}    return Ok(row)\n\
         }}\n\n"
    )
}

/// Returns the literal one field is seeded with.
fn seed(field: &Field) -> &'static str {
    if field.nullable || field.generated || is_timestamp(field) {
        // Left nil so the database assigns it and validation can tell the difference
        // between "not supplied" and "deliberately empty".
        return "nil";
    }
    field.kind.zero()
}

/// Returns `true` when the field is an audit timestamp the database maintains.
pub(super) fn is_timestamp(field: &Field) -> bool {
    matches!(field.column.as_str(), "created_at" | "updated_at")
}
