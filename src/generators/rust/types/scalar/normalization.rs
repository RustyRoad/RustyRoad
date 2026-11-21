//! PostgreSQL array and precision-modifier spelling normalization.

/// Resolve native array spellings while preserving the element type.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn array_element(lowered: &str) -> Option<String> {
    if let Some(element) = lowered.strip_suffix("[]") {
        return Some(element.trim().to_string());
    }
    lowered
        .strip_prefix('_')
        .map(|element| element.trim().to_string())
}

/// Remove precision modifiers without dropping a timestamp timezone suffix.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust::types) fn strip_modifier(lowered: &str) -> String {
    let Some(start) = lowered.find('(') else {
        return lowered.trim().to_string();
    };
    let Some(end) = lowered.rfind(')') else {
        return lowered.trim().to_string();
    };

    format!("{}{}", &lowered[..start], &lowered[end + 1..])
        .trim()
        .to_string()
}
