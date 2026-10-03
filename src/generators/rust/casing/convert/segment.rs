//! Splitting a database name into word segments.

/// Splits a database name into lower-case `snake_case` segments.
///
/// Shared by both conversions so they always agree on where the word boundaries are; each
/// then repairs a leading digit in the way its own target requires.
pub(super) fn segments(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    let mut previous_lower = false;

    for character in name.chars() {
        if character == '_' || character == '-' || character == ' ' {
            if !out.ends_with('_') && !out.is_empty() {
                out.push('_');
            }
            previous_lower = false;
            continue;
        }

        if character.is_uppercase() {
            if previous_lower {
                out.push('_');
            }
            out.extend(character.to_lowercase());
            previous_lower = false;
            continue;
        }

        // Punctuation carries no word boundary of its own and cannot appear in an
        // identifier, so it is dropped rather than mapped to an underscore.
        if !character.is_alphanumeric() {
            continue;
        }

        out.push(character);
        previous_lower = character.is_lowercase() || character.is_ascii_digit();
    }

    out.trim_matches('_').to_string()
}
