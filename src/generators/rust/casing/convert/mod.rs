//! Case conversion shared by field and type names.

mod segment;

use segment::segments;

/// Converts a database name to `snake_case`.
///
/// Handles both `snake_case` input, which passes through, and `camelCase` input, where a
/// case boundary becomes an underscore.
pub fn to_snake(name: &str) -> String {
    let split = segments(name);

    if split.is_empty() {
        return "field".to_string();
    }
    // An identifier cannot begin with a digit, and prefixing keeps the original name
    // readable where truncating would not.
    if split.starts_with(|c: char| c.is_ascii_digit()) {
        return format!("f_{split}");
    }
    split
}

/// Converts a database name to `PascalCase`.
pub fn to_pascal(name: &str) -> String {
    // Segmented directly rather than via `to_snake`, whose digit prefix is chosen for a
    // field name and would surface here as a stray leading `F`.
    let split = segments(name);
    let mut out = String::with_capacity(split.len());

    for part in split.split('_').filter(|part| !part.is_empty()) {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }

    if out.is_empty() {
        return "Row".to_string();
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        return format!("T{out}");
    }
    out
}
