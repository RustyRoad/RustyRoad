//! Shared SQL and literal fragments.

use crate::generators::tetherscript::model::Field;

/// Renders a comma-separated column list, one column per line.
pub fn indented_columns(fields: &[&Field], spaces: usize) -> String {
    let indent = " ".repeat(spaces);

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
pub fn assignments(fields: &[&Field], start: usize, spaces: usize) -> String {
    let indent = " ".repeat(spaces);

    fields
        .iter()
        .enumerate()
        .map(|(offset, field)| format!("{indent}{} = ${}", quote(&field.column), start + offset))
        .collect::<Vec<_>>()
        .join(",\n")
}

/// Renders the bind list a query passes as its parameter array.
///
/// Parameters are bound rather than spliced, which is what keeps a hostile value
/// inert: it can never change the shape of the statement.
pub fn binds(fields: &[&Field]) -> String {
    fields
        .iter()
        .map(|field| format!("row[\"{}\"]", field.column))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Quotes a column name when Postgres would not accept it bare.
///
/// An unquoted mixed-case column is folded to lower case by Postgres and the
/// statement then fails, so anything not plainly lower snake_case is quoted.
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
