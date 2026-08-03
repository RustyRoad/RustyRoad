//! Postgres type to JSON Schema type and format mapping.

/// Maps a Postgres type to its JSON Schema type.
pub(super) fn base_type(sql_type: &str) -> &'static str {
    let lowered = sql_type.to_lowercase();

    if lowered.starts_with("bool") {
        return "boolean";
    }
    if lowered.starts_with("json") {
        return "object";
    }
    // numeric is a string to avoid float precision loss, matching Drizzle.
    if lowered.starts_with("numeric") || lowered.starts_with("decimal") {
        return "string";
    }
    if lowered.starts_with("real") || lowered.starts_with("double") {
        return "number";
    }

    let integer = ["smallint", "integer", "int", "bigint", "serial"]
        .iter()
        .any(|prefix| lowered.starts_with(prefix));

    if integer {
        "integer"
    } else {
        "string"
    }
}

/// Returns the JSON Schema `format` for a type, when one applies.
///
/// A format is advisory, but Hey API surfaces it in the generated types and it
/// documents the wire shape for anyone reading the document.
pub(super) fn format_of(sql_type: &str) -> Option<&'static str> {
    let lowered = sql_type.to_lowercase();

    // Checked before `date`/`time` so a timestamp is not matched as either.
    if lowered.starts_with("timestamp") {
        return Some("date-time");
    }
    if lowered.starts_with("uuid") {
        return Some("uuid");
    }
    if lowered.starts_with("date") {
        return Some("date");
    }
    if lowered.starts_with("time") {
        return Some("time");
    }

    None
}
