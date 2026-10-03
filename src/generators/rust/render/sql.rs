//! Shared SQL fragment rendering.
//!
//! The column list an insert names and the placeholders it binds have to agree, so
//! both are produced from the same field slice here rather than being written out
//! in each renderer.

use crate::generators::rust::model::Field;

/// Renders a comma-separated column list, one column per line.
///
/// Indented to sit inside a raw string in the generated file, matching the
/// hand-written models where a wide insert stays readable.
pub fn indented_columns(fields: &[&Field], tabs: usize) -> String {
    let indent = "\t".repeat(tabs);

    fields
        .iter()
        .map(|field| format!("{indent}{}", quote(&field.column)))
        .collect::<Vec<_>>()
        .join(",\n")
}

/// Renders `$1, $2, ...` for `count` bind parameters.
pub fn placeholders(count: usize) -> String {
    (1..=count)
        .map(|index| format!("${index}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Renders `SET` assignments for an update, numbering from `start`.
pub fn assignments(fields: &[&Field], start: usize, tabs: usize) -> String {
    let indent = "\t".repeat(tabs);

    fields
        .iter()
        .enumerate()
        .map(|(offset, field)| format!("{indent}{} = ${}", quote(&field.column), start + offset))
        .collect::<Vec<_>>()
        .join(",\n")
}

/// Quotes a column name when Postgres would not accept it bare.
///
/// An unquoted mixed-case or keyword column is folded to lower case by Postgres and
/// the statement then fails, so anything not plainly lower snake_case is quoted.
pub fn quote(column: &str) -> String {
    let plain = !column.is_empty()
        && !column.starts_with(|c: char| c.is_ascii_digit())
        && column
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');

    if plain {
        return column.to_string();
    }
    format!("\\\"{}\\\"", column.replace('"', ""))
}
