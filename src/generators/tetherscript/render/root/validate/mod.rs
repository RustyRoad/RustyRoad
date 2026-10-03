//! `validate`, the stand-in for the field types the language does not have.

mod loops;

use crate::generators::tetherscript::model::Model;
use loops::{KIND_LOOP, REQUIRED_LOOP};
use std::fmt::Write as _;

/// Renders the validation function.
pub(super) fn render(model: &Model) -> String {
    let mut body = required(model);
    body.push('\n');
    body.push_str(&kinds(model));

    format!(
        "// Checks a row before it reaches SQL.\n\
         //\n\
         // A dynamically-typed row can hold anything, so the kind each column decodes\n\
         // to is checked here. Catching it now names the offending column; letting it\n\
         // reach Postgres produces an error about a parameter number instead.\n\
         fn validate(row) {{\n{body}    return Ok(true)\n\
         }}\n\n"
    )
}

/// Renders the required-column check.
fn required(model: &Model) -> String {
    let required = model.required();

    if required.is_empty() {
        // Nothing is required, so the loop would be dead code.
        return "    // No column is both required and caller-supplied.\n".to_string();
    }

    let listed = required
        .iter()
        .map(|field| format!("\"{}\"", field.column))
        .collect::<Vec<_>>()
        .join(", ");

    format!("    let required = [{listed}]\n{REQUIRED_LOOP}")
}

/// Renders the per-column kind checks.
fn kinds(model: &Model) -> String {
    let checked = model.insertable();

    if checked.is_empty() {
        return String::new();
    }

    let mut body = String::from("    let kinds = map()\n");
    for field in &checked {
        let column = &field.column;
        let _ = writeln!(body, "    kinds[\"{column}\"] = \"{}\"", field.kind.name());
    }
    body.push_str(KIND_LOOP);

    body
}
