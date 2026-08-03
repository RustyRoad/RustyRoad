//! Applying casing rules to produce identifiers and bindings.

use super::{convert, Casing};

/// Converts a database name to an object key, quoting it when necessary.
pub fn identifier(name: &str, casing: Casing) -> String {
    escape(&casing.apply(name))
}

/// Converts a database name to a binding name, always valid unquoted.
///
/// A `const` declaration cannot be quoted, so an invalid name is repaired.
pub fn binding(name: &str, casing: Casing) -> String {
    let converted = casing.apply(name);
    if convert::is_valid_identifier(&converted) {
        return converted;
    }
    convert::repair(&converted)
}

/// Converts a name to `PascalCase`, used for generated type names.
pub fn to_pascal(name: &str) -> String {
    let camel = binding(name, Casing::Camel);
    let mut chars = camel.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => camel,
    }
}

/// Quotes an object key when it is not a valid bare TypeScript identifier.
pub fn escape(name: &str) -> String {
    if convert::is_valid_identifier(name) {
        name.to_string()
    } else {
        format!("\"{}\"", name.replace('"', "\\\""))
    }
}

/// Returns the database-name argument for a column builder.
///
/// Under `preserve` the identifier already equals the database name, so Drizzle
/// omits the redundant argument; under `camel` it must be stated.
pub fn db_name_argument(name: &str, casing: Casing) -> String {
    match casing {
        Casing::Preserve => String::new(),
        Casing::Camel => format!("\"{name}\""),
    }
}
