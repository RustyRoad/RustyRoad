//! Name conversion between database and TypeScript conventions.

/// Converts `snake_case` or `kebab-case` to `camelCase`.
pub fn to_camel(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut capitalize = false;

    for character in name.chars() {
        if character == '_' || character == '-' || character == ' ' {
            capitalize = !out.is_empty();
            continue;
        }
        if capitalize {
            out.extend(character.to_uppercase());
            capitalize = false;
        } else {
            out.push(character);
        }
    }

    out
}

/// Returns `true` when `name` can appear unquoted in TypeScript.
pub(super) fn is_valid_identifier(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

/// Repairs a name that cannot appear as a `const` binding.
///
/// A declaration cannot be quoted, so invalid characters are dropped and a
/// digit-leading name is prefixed rather than escaped.
pub(super) fn repair(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '$')
        .collect();

    if cleaned.is_empty() {
        return "table".to_string();
    }
    if cleaned.starts_with(|c: char| c.is_ascii_digit()) {
        return format!("t{cleaned}");
    }
    cleaned
}
