//! Response rendering for one operation.

use super::super::operations::Operation;

/// Renders the response map.
pub(super) fn responses(operation: &Operation) -> String {
    let mut entries = vec![success(operation)];
    if operation.can_be_missing {
        entries.push(not_found());
    }

    format!(
        "        \"responses\": {{\n{}\n        }}",
        entries.join(",\n")
    )
}

/// Renders the success response.
fn success(operation: &Operation) -> String {
    let code = operation.success;

    // A 204 carries no body, so no content is declared.
    let Some(row) = operation.row_type.as_deref() else {
        return format!("          \"{code}\": {{ \"description\": \"No content\" }}");
    };

    let schema = if operation.is_collection {
        format!(
            "{{ \"type\": \"array\", \"items\": {{ \"$ref\": \"#/components/schemas/{row}\" }} }}"
        )
    } else {
        format!("{{ \"$ref\": \"#/components/schemas/{row}\" }}")
    };

    format!(
        "          \"{code}\": {{\n\
         \x20           \"description\": \"Success\",\n\
         \x20           \"content\": {{ \"application/json\": {{ \"schema\": {schema} }} }}\n\
         \x20         }}"
    )
}

/// Renders the 404 returned when a row is missing.
fn not_found() -> String {
    "          \"404\": {\n\
     \x20           \"description\": \"Not found\",\n\
     \x20           \"content\": { \"application/json\": { \"schema\": { \"$ref\": \"#/components/schemas/ApiError\" } } }\n\
     \x20         }"
        .to_string()
}
