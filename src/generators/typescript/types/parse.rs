//! Postgres type name normalization.

/// Extracts the text inside a type's parentheses.
pub(super) fn modifier(lowered: &str) -> Option<String> {
    let start = lowered.find('(')?;
    let end = lowered.rfind(')')?;
    if end <= start + 1 {
        return None;
    }
    Some(lowered[start + 1..end].trim().to_string())
}

/// Removes a parenthesized modifier wherever it appears in the type name.
///
/// A precision modifier sits mid-type in `timestamp(3) with time zone`, so trimming
/// at the first parenthesis would leave the suffix behind and break matching.
pub(super) fn strip_modifier(lowered: &str) -> String {
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
